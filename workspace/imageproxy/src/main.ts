import { fastify } from 'fastify'
import sharp from 'sharp'
import { Readable } from 'node:stream'
import { encode } from 'blurhash'

interface QueryOpts {
  u: string | undefined
  w: string | undefined
  h: string | undefined
  f: string | undefined
  ch: string | undefined
  cw: string | undefined
  q: string | undefined
}

type Resize = {
  width: number | undefined
  height: number | undefined
}

async function main() {
  const supportedUrls = process.env.SUPPORTED_URLS?.split(',') || []
  const server = fastify()
  server.setErrorHandler((error, request, reply) => {
    console.error({ error, request }, 'uncaught error')
    reply.status(500).send({ ok: false })
  })
  server.get('/healthcheck', {}, async function(_, reply) {
    reply.code(200).send({ok: true})
  })
  server.get('/imageproxy', {}, async function (request, reply) {
    const opts = request.query as QueryOpts
    if (!opts.u) {
      reply.code(400).send()
      return
    }
    const imageUrl = new URL(opts.u)
    let isSupported = false
    for (const supported of supportedUrls) {
      if (opts.u.startsWith(supported)) {
        isSupported = true
        break
      }
    }
    if (!isSupported) {
      reply.code(401).send()
      return
    }
    const response = await fetch(imageUrl)
    if (!response.ok || !response.body) {
      reply.code(500).send()
      return
    }
    let contentType = response.headers.get('Content-Type')
    let transformer = sharp()
    // @ts-ignore
    if (opts.w || opts.h) {
      let resize: Resize = { width: undefined, height: undefined }
      if (opts.w) {
        resize.width = parseInt(opts.w)
        if (isNaN(resize.width)) {
          return reply.code(400).send()
        }
      }
      if (opts.h) {
        resize.height = parseInt(opts.h)
        if (isNaN(resize.height)) {
          return reply.code(400).send()
        }
      }
      transformer = transformer.resize(resize)
    }
    switch (opts.f) {
      case 'blurhash': {
        transformer = transformer.raw().ensureAlpha()
        // @ts-ignore
        const result = await Readable.fromWeb(response.body).pipe(transformer).toBuffer({ resolveWithObject: true })
        const img = Uint8ClampedArray.from(result.data)
        const cWidth = opts.cw ? parseInt(opts.cw) : 4
        const cHeight = opts.ch ? parseInt(opts.ch) : 4
        const blurhash = encode(img, result.info.width, result.info.height, cWidth, cHeight)
        return reply
          .header('Content-Type', 'text/plain')
          .send(blurhash)
      }
      case 'jpeg': {
        contentType = 'image/jpeg'
        const quality = opts.q ? parseInt(opts.q) : 80
        if (isNaN(quality)) {
          return reply.code(400).send()
        }
        transformer = transformer.toFormat(sharp.format.jpeg, {
          quality: quality,
          mozjpeg: true,
        })
        break
      }
      case 'webp': {
        contentType = 'image/webp'
        const quality = opts.q ? parseInt(opts.q) : 80
        if (isNaN(quality)) {
          return reply.code(400).send()
        }
        transformer = transformer.toFormat(sharp.format.webp, {
          quality: quality,
        })
        break
      }
    }
    // @ts-ignore
    const buffer = await Readable.fromWeb(response.body).pipe(transformer).toBuffer()
    await reply.header('Content-Type', contentType).send(buffer)
  })
  await server.listen({ host: '0.0.0.0', port: 8003 })
  console.log('server listening on 0.0.0.0:8003')
}

void main()
