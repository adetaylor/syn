use crate::error::Result;
use proc_macro2::TokenStream;
use std::fs;
use std::io::Write;
use std::path::Path;

pub fn write<P: AsRef<Path>>(path: P, content: TokenStream) -> Result<()> {
    let mut formatted = Vec::new();
    write!(
        formatted,
        "// THIS FILE IS AUTOMATICALLY GENERATED; DO NOT EDIT\n\n"
    )?;

    let mut config = rustfmt::Config::default();
    config.set().emit_mode(rustfmt::EmitMode::Stdout);
    config.set().verbose(rustfmt::Verbosity::Quiet);
    config.set().format_macro_matchers(true);
    config.set().normalize_doc_attributes(true);

    let mut session = rustfmt::Session::new(config, Some(&mut formatted));
    session.format(rustfmt::Input::Text(content.to_string()))?;
    drop(session);

    if path.as_ref().is_file() && fs::read(&path)? == formatted {
        return Ok(());
    }

    fs::write(path, formatted)?;
    Ok(())
}
