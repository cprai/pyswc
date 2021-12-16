use std::path::Path;
use std::sync::Arc;
use swc::Compiler;
use swc::config::{JsMinifyOptions, JsMinifyFormatOptions};
use swc::config::util::BoolOrObject;
use swc_common::SourceMap;
use swc_common::errors::{ColorConfig, Handler};
use swc_ecma_minifier::option::terser::TerserEcmaVersion;


fn main() {
    let cm = Arc::<SourceMap>::default();
    let handler = Arc::new(Handler::with_tty_emitter(
        ColorConfig::Auto,
        true,
        false,
        Some(cm.clone()),
    ));
    let c = Compiler::new(cm.clone());

    let fm = cm
        .load_file(Path::new("foo.js"))
        .expect("failed to load file");

    let transformed = c.minify(
        fm,
        &handler,
        &JsMinifyOptions {
            compress: true.into(),
            mangle: true.into(),
            format: JsMinifyFormatOptions {
                ascii_only: false,
                beautify: false,
                braces: true,
                comments: false.into(),
                ecma: 5,
                indent_level: 4,
                indent_start: false,
                inline_script: false,
                keep_numbers: true,
                keep_quoted_props: true,
                max_line_len: BoolOrObject::Obj(80),
                preamble: String::from(""),
                quote_keys: true,
                quote_style: 3,
                preserve_annotations: true,
                safari10: false,
                semicolons: true,
                shebang: false,
                webkit: false,
                wrap_iife: true,
                wrap_func_args: true,
            },
            ecma: TerserEcmaVersion::Num(5),
            keep_classnames: true,
            keep_fnames: true,
            module: false,
            safari10: false,
            toplevel: false,
            source_map: false.into(),
            output_path: None,
            inline_sources_content: true,
        },
    )
    .expect("failed to process file");
    print!("{}\n", transformed.code);
}
