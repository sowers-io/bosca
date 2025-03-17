use crate::context::BoscaContext;
use async_graphql::{Context, Error, Upload};
use axum::extract::multipart::Field;
use bytes::{Buf, BufMut, BytesMut};
use futures_util::AsyncReadExt;
use log::error;
use object_store::path::Path;
use std::io::Write;

pub async fn upload_file(
    ctx: &BoscaContext,
    graphql_ctx: &Context<'_>,
    path: Path,
    file: Upload,
) -> Result<usize, Error> {
    let mut len = 0;
    if file.0 < 5242880 {
        let mut content = file.value(graphql_ctx)?.into_async_read();
        let mut buf = Vec::with_capacity(file.0);
        len = content.read_to_end(&mut buf).await?;
        ctx.storage.put(&path, buf.into()).await?;
    } else {
        let mut multipart = ctx.storage.put_multipart(&path).await?;
        let mut content = file.value(graphql_ctx)?.into_async_read();
        let mut buf = vec![0_u8; 524288];
        loop {
            let read = content.read(&mut buf).await?;
            if read > 0 {
                len += read;
                let buf_slice = buf[..read].to_vec();
                multipart.put_part(buf_slice.into()).await?;
            } else {
                multipart.complete().await?;
                break;
            }
        }
    }
    Ok(len)
}

pub async fn upload_field(
    ctx: &BoscaContext,
    path: Path,
    field: &mut Field<'_>,
) -> Result<usize, Error> {
    let mut upload = ctx.storage.put_multipart(&path).await?;
    let mut len = 0;
    let buf = BytesMut::with_capacity(5242880);
    let writer = &mut buf.writer();
    while let Some(chunk) = field.chunk().await? {
        let chunk_len = chunk.len();
        len += chunk_len;
        let write_len = writer.write(chunk.as_ref())?;
        if write_len != chunk_len {
            error!("Error validating write {}, {}", write_len, chunk_len);
            return Err(Error::new(format!(
                "invalid length: {} != {}",
                write_len, chunk_len
            )));
        }
        let buf_len = writer.get_ref().len();
        if buf_len >= 5242880 {
            let copy = writer.get_mut().copy_to_bytes(buf_len);
            writer.get_mut().clear();
            upload.put_part(copy.into()).await?;
        }
    }
    let buf_len = writer.get_ref().len();
    if buf_len > 0 {
        let copy = writer.get_mut().copy_to_bytes(buf_len);
        upload.put_part(copy.into()).await?;
    }
    upload.complete().await?;
    Ok(len)
}
