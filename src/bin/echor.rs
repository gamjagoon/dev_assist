use clap::{Arg, ArgAction, Command};

#[cfg(target_os = "windows")]
pub const NEWLINE: &str = "\r\n";
#[cfg(target_os = "linux")]
pub const NEWLINE: &str = "\n";

fn main() {
    let matches = Command::new("echor")
        .version("0.1.0")
        .author("rlaalswo201@gmail.com")
        .about("Rust Version of `echo`")
        .arg(
            Arg::new("text")
                .value_name("TEXT")
                .help("Input text")
                .required(true)
                .num_args(1..),
        )
        .arg(
            Arg::new("omit_newline")
                .short('n')
                .action(ArgAction::SetTrue)
                .help("Do notg print newline"),
        )
        .get_matches();

    let text: Vec<String> = matches
        .get_many("text")
        .expect("Clone InPut Text Error")
        .cloned()
        .collect();

    let omit_newline = matches.get_flag("omit_newline");

    let ending = if omit_newline { "" } else { NEWLINE };

    print!("{}{ending}", text.join(" "));
}
