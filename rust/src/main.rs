mod compile;

use std::error::Error;
use std::fs;
use std::time::Duration;

use clap::Parser;

use crate::compile::token::{Punct, TokenList};
use crate::compile::tree::{build_tree, ExprTree, VisitOrder};
use crate::compile::types::avoid_warnings;

#[derive(clap::Parser, Debug)]
#[clap(author, version, about, long_about = None)]
struct Args {
	#[clap(subcommand)]
	command: SubCommand,
}

#[derive(clap::Subcommand, Debug)]
enum SubCommand {
	Dev {
		/// more debug testing, also to avoid unused code warning
		#[clap(short, long)]
		debug: bool,

		/// expression
		#[clap(value_parser, default_value = "(1 + 2) * ((3 - 5) * 2) ^ 2 + 2 * 6")]
		expr: String,
	},
	Lex {
		#[clap(short, long, action)]
		file: Option<String>,

		#[clap(value_parser, default_value = "char c = 'A', d = 'C'; int i = 1, j, k = 3;")]
		cli_text: String,
	},
	Parse {
		#[clap(short, long, action, default_value = "data/t0.c")]
		file: String,

		#[clap(short, long)]
		debug: bool,
	},
	Http,
	Term,
}

#[test]
fn test_progress_bar() {
	let total = 64 << 10;
	let count = 1024 * 8;
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
}

fn main() -> Result<(), Box<dyn Error>> {
	let args = Args::parse();

	match args.command {
		SubCommand::Dev { expr, debug } => {
			let tree = build_tree(&expr)?;

			if debug {
				tree.print(&VisitOrder::Pre);
				println!("─────");
				tree.print(&VisitOrder::In);
				println!("─────");
				tree.print(&VisitOrder::Post);
				println!("eval stack to {}", tree.eval_stack());
				let tree = ExprTree::tree(Punct::Add, ExprTree::branch(Punct::Mul, 1, 2), ExprTree::leaf(3));
				tree.print(&VisitOrder::Pre);
			}

			println!("──────\n{}\n{}= {}", expr, tree, tree.eval());
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
			let input = if let Some(f) = file { fs::read_to_string(f)? } else { cli_text };
			if let Ok(r) = input.as_str().parse::<TokenList>() {
				println!("lex: {}\n{:#}{}", input, r, r);
			}
		}
		SubCommand::Parse { file, debug } => {
			let src = fs::read_to_string(file)?;

			if debug {
				println!("{}\n\n\n", src);
				avoid_warnings();

				match compile::parse::Parser::from_str(src.as_str()).and_then(|mut p| p.test_global_variable()) {
					Ok(Some(declspec)) => println!("------\n{:?}", declspec),
					Ok(None) => println!("------\nNone\n"),
					Err(e) => println!("\t[error]\n{}", e),
				}
			}

			match compile::parse::Parser::test(src.as_str()) {
				Ok(expr) => println!("------\n{}", expr),
				Err(e) => println!("\t[error]\n{}", e),
			}
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
