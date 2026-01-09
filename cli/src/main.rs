use std::{
  fs,
  io::{self, Read, Result, Write},
  path::PathBuf,
};

use clap::Parser;
use rsdm_parser::MarkdownParser;

#[derive(Debug, Parser)]
#[command(name = "rsmd", bin_name="rsmd", version, about, long_about = None)]
struct Args {
  #[arg(short, long)]
  out: Option<PathBuf>,

  file: PathBuf,
}

fn main() -> Result<()> {
  let args = Args::parse();

  let mut input_file = fs::File::open(&args.file)?;
  let mut contents = String::new();
  input_file.read_to_string(&mut contents)?;

  let parser = MarkdownParser::with_defaults();
  let html = parser.parse_to_html(&contents);

  if let Some(out) = &args.out {
    let output_path = if out.is_dir() || out.to_string_lossy().ends_with(std::path::MAIN_SEPARATOR) {
      let file_name = args
        .file
        .file_name()
        .ok_or_else(|| io::Error::new(io::ErrorKind::InvalidInput, "Input path has no file name"))?;
      out.join(file_name).with_extension("html")
    } else {
      out.to_path_buf()
    };

    if let Some(parent) = output_path.parent() {
      fs::create_dir_all(parent)?;
    }

    let mut output_file = fs::File::create(&&output_path)?;
    output_file.write_all(html.as_bytes())?;
    println!("Markdown written to {:?}", output_path);
  } else {
    println!("{}", html);
  }

  Ok(())
}
