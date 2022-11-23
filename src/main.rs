mod encoder;
mod decoder;
mod utils;
mod png_io;
mod header;

use std::{path::Path, fmt::format};
use image::ImageFormat;
use utils::print_vector;

use crate::png_io::{read_image_data_from_file, write_image};

use clap::{Args, Parser, Subcommand};

#[derive(Parser)]
#[command(author, version, about, long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[command(subcommand)]
    command: Commands
}

#[derive(Subcommand)]
enum Commands {
    Encode(Encode),
    Decode(Decode),
}

#[derive(Args)]
struct Encode {
   input: String,
   text: String,
   #[arg(short, long)]
   output: Option<String>,
}

#[derive(Args)]
struct Decode {
   input: String,
}




fn main(){
    let cli = Cli::parse();
    match &cli.command {
        Commands::Encode(cmd) => {
            encode(&cmd.input, &cmd.text, &cmd.output);
        },
        Commands::Decode(cmd) => {
            decode(&cmd.input);
        },
    }

}

fn encode(input: &str, text: &str, specified_output: &Option<String>) {
    let output: String;
    if let Some(specified_output) = specified_output.as_deref() {
        output = String::from(specified_output);
    }
    else {
        let input_path = Path::new(&input);
        output = get_output_filename(&input_path);
    }
    let source_img =  read_image_data_from_file(&input);
    let format = ImageFormat::from_path(&input).unwrap();
    let encoded_text = encoder::encode_text(text, &source_img.bytes, format);
    write_image(&output, source_img, encoded_text);
}

fn decode (input: &str) {
    let source_img = read_image_data_from_file(input);
    let out_text = decoder::decode_text(&source_img.bytes);
    println!("{out_text}");
    
}


fn get_output_filename(input: &Path) -> String {
    let file_name = input.file_stem().unwrap().to_str().unwrap();
    let ext = input.extension().unwrap().to_str().unwrap();

    return format!("{file_name}-out.{ext}");
}
