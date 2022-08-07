use std::time::Duration;

use clap::Parser;

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
    Term,
    Lex,
    Parse,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let args = Args::parse();

    match args.command {
        SubCommand::Dev { name: _, count } => {
            let total = 64 << 10;
            let pb = indicatif::ProgressBar::new(total);
            let tmpl = " {spinner:.green} [{elapsed_precise}] [{wide_bar:.cyan/blue}] {bytes}/{total_bytes} ({eta})";
            pb.set_style(
                indicatif::ProgressStyle::with_template(tmpl)
                    .unwrap()
                    .with_key(
                        "eta",
                        |state: &indicatif::ProgressState, w: &mut dyn std::fmt::Write| {
                            write!(w, "{:.3}s", state.eta().as_secs_f64()).unwrap()
                        },
                    )
                    .progress_chars("#>-"),
            );

            for i in (0..total).step_by(count as usize) {
                std::thread::sleep(Duration::from_millis(50));
                // if i % 50 == 0 { pb.println(format!("[+] #{}", i)); }
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
        }
        SubCommand::Lex => {
            use dialoguer::{theme::ColorfulTheme, Confirm};

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you want to continue?")
                .interact()
                .unwrap()
            {
                println!("Looks like you want to continue");
            } else {
                println!("nevermind then :(");
            }

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you really want to continue?")
                .default(true)
                .interact()
                .unwrap()
            {
                println!("Looks like you want to continue");
            } else {
                println!("nevermind then :(");
            }

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you really really want to continue?")
                .default(true)
                .show_default(false)
                .wait_for_newline(true)
                .interact()
                .unwrap()
            {
                println!("Looks like you want to continue");
            } else {
                println!("nevermind then :(");
            }

            if Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you really really really want to continue?")
                .wait_for_newline(true)
                .interact()
                .unwrap()
            {
                println!("Looks like you want to continue");
            } else {
                println!("nevermind then :(");
            }

            match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you really really really really want to continue?")
                .interact_opt()
                .unwrap()
            {
                Some(true) => println!("Looks like you want to continue"),
                Some(false) => println!("nevermind then :("),
                None => println!("Ok, we can start over later"),
            }

            match Confirm::with_theme(&ColorfulTheme::default())
                .with_prompt("Do you really really really really really want to continue?")
                .default(true)
                .wait_for_newline(true)
                .interact_opt()
                .unwrap()
            {
                Some(true) => println!("Looks like you want to continue"),
                Some(false) => println!("nevermind then :("),
                None => println!("Ok, we can start over later"),
            }
        }
        SubCommand::Parse => {
            println!("parse");
        }
    }

    Ok(())
}
