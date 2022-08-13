mod lex;
mod parse;

use std::error::Error;
use std::fs;
use std::time::Duration;

use clap::Parser;

use lex::*;
use parse::*;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(subcommand)]
	command: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
enum SubCommand {
	Dev {
		/// Name of the person to greet
		#[clap(short, long, default_value = "dev")]
		name: String,

		/// Number of times to greet
		#[clap(short, long, default_value_t = 1)]
		count: u16,
	},
	Lex {
		#[clap(short, long, action)]
		file: Option<String>,

		#[clap(value_parser)]
		cli_text: Option<String>,
	},
	Parse {
		#[clap(short, long, action, default_value = "data/t0.c")]
		file: String,
	},
	Http,
	Term,
}

fn main() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();

	match args.command {
		SubCommand::Dev { name: _, count } => {
			let total = 64 << 10;
			let pb = indicatif::ProgressBar::new(total);
			let tmpl = " {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
			pb.set_style(
				indicatif::ProgressStyle::with_template(tmpl)
					.unwrap()
					.with_key("eta", |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
						write!(w, "{:.3}s", state.eta().as_secs_f64()).unwrap()
					})
					.progress_chars("#>-"),
			);

			for i in (0..total).step_by(count as usize) {
				std::thread::sleep(Duration::from_millis(100));
				pb.set_position(i);
			}
			pb.finish_with_message("done");

			use console::style;
			println!("This is {} neat", style("quite").cyan());
		}

		SubCommand::Term => {
			use console::Term;
			let term = Term::stdout();
			term.clear_screen()?;
			term.move_cursor_down(3)?;
			term.write_line("hi in line 3")?;
			term.move_cursor_down(3)?;
			term.write_line("hi in line 6")?;
			term.write_line("I will disappeare after 3 seconds")?;
			std::thread::sleep(Duration::from_secs(3));
			term.move_cursor_up(1)?;
			term.clear_line()?;
			term.move_cursor_down(3)?;
		}
		SubCommand::Lex { file, cli_text } => {
			let input = if let Some(f) = file { fs::read_to_string(f)? } else { cli_text.ok_or("input is empty")? };
			println!("lex: {}\n{:#?}", input, TokenApi::parse_all(input.as_str()));
		}
		SubCommand::Parse { file } => {
			let src = fs::read_to_string(file)?;
			println!("{}\n\n\n", src);
			println!("{:#?}", SyntaxTree::compile(src.as_str()));
		}
		SubCommand::Http => {
			use http::Request;
			use serde::ser;

			fn serialize<T>(req: Request<T>) -> serde_json::Result<Request<Vec<u8>>>
			where
				T: ser::Serialize,
			{
				let (parts, body) = req.into_parts();
				let body = serde_json::to_vec(&body)?;
				Ok(Request::from_parts(parts, body))
			}

			let req = Request::builder()
				.uri("http://zhoushen929.com?q=love#id")
				.header("User-Agent", "rust rocks!")
				.body(())
				.unwrap();
			println!(
				"{:#?}\n {:?}\n{:#?}, {:#?}, {:#?}, {:#?}",
				req,
				req.headers(),
				req.uri().host(),
				req.uri().scheme(),
				req.uri().path(),
				req.uri().query(),
			);
			let r = serialize(req).unwrap();
			// ghci> map fromEnum "null"
			// [110,117,108,108]
			println!("serialize: {:#?}", r);
		}
	}

	Ok(())
}