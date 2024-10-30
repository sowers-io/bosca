use swc_common::{BytePos, GLOBALS};
use swc_common::input::StringInput;
use swc_css::ast::Stylesheet;

use swc_css::parser::lexer::Lexer as CSSLexer;
use swc_css::parser::Parse;
use swc_css::parser::parser::{Parser as CSSParser, ParserConfig as CSSParserConfig};
use swc_css_codegen::writer::basic::BasicCssWriter;
use swc_css_codegen::Emit as _;
use swc_css_minifier::minify;

pub fn minify_css(css: String) -> String {
    let buf = css.as_str();
    let cfg = CSSParserConfig {
        allow_wrong_line_comments: true,
        css_modules: true,
        legacy_nesting: true,
        legacy_ie: false,
    };
    let lexer = CSSLexer::new(StringInput::new(buf, BytePos(0), BytePos(buf.len() as u32)), None, cfg);
    let mut parser = CSSParser::new(lexer, cfg);
    let mut stylesheet: Stylesheet = parser.parse().unwrap();
    GLOBALS.set(&Default::default(), || {
        minify(&mut stylesheet, swc_css_minifier::options::MinifyOptions {});
    });
    let mut buf = String::new();
    {
        let wr = BasicCssWriter::new(&mut buf, None, Default::default());
        let mut generator = swc_css_codegen::CodeGenerator::new(
            wr,
            swc_css_codegen::CodegenConfig {
                minify: true,
            },
        );
        generator.emit(&stylesheet).unwrap();
    }
    buf
}