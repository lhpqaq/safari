use clap::Command;
mod internel;

fn main() {
    let matches = Command::new("Safari CheckPoint")
        .about("Save and restore Safari windows and tabs")
        .subcommand(Command::new("dump").about("Dump Safari windows and tab"))
        .subcommand(Command::new("reopen").about("Reopen Safari windows and tabs"))
        .subcommand(Command::new("list").about("List Safari windows and tabs"))
        .get_matches();

    match matches.subcommand() {
        Some(("dump", _sub_matches)) => {
            internel::dump();
        }
        Some(("reopen", _sub_matches)) => {
            internel::reopen();
        }
        Some(("list", _sub_matches)) => {
            internel::list();
        }
        _ => eprintln!("Invalid command! Use 'dump' or 'reopen' or 'list'."),
    }
}
