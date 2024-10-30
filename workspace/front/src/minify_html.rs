use swc_common::BytePos;
use swc_common::input::StringInput;
use swc_html::parser::lexer::Lexer as HTMLLexer;
use swc_html::parser::parser::{Parser as HTMLParser, ParserConfig as HTMLParserConfig};
use swc_html_codegen::Emit as _;
use swc_html_codegen::writer::basic::BasicHtmlWriter;
use swc_html_minifier::minify_document;
use swc_html_minifier::option::{CollapseWhitespaces, CssOptions, MinifyOptions};

pub fn minify_html(html: String) -> String {
    let buf = html.as_str();
    let lexer = HTMLLexer::new(StringInput::new(buf, BytePos(0), BytePos(buf.len() as u32)));
    let mut parser = HTMLParser::new(lexer, HTMLParserConfig {
        scripting_enabled: true,
        iframe_srcdoc: false,
        allow_self_closing: true,
    });
    let mut document = parser.parse_document().unwrap();
    let options: MinifyOptions<CssOptions> = MinifyOptions::<CssOptions> { collapse_whitespaces: CollapseWhitespaces::Conservative, ..Default::default() };
    minify_document(&mut document, &options);
    let mut buf = String::new();
    {
        let wr = BasicHtmlWriter::new(&mut buf, None, Default::default());
        let mut generator = swc_html_codegen::CodeGenerator::new(
            wr,
            swc_html_codegen::CodegenConfig {
                minify: true,
                ..Default::default()
            },
        );
        generator.emit(&document).unwrap();
    }
    buf
}
