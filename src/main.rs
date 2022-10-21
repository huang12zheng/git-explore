use clap::Parser;
use git_explore::*;

fn main() {
    let cli = RepoCli::parse();
    // let cli = RepoCli::parse_from([KEY_COMMAND, "list", "-d", "d:\\rust\\backend\\sdk1018"]);
    // println!("cli: {:#?}", cli);
    match cli.command {
        Some(Command::List(opt)) => run_list(&opt).unwrap(),
        Some(Command::Init(opt)) => init(&opt).unwrap(),
        Some(Command::Commit(mut opt)) => commit(&mut opt).unwrap(),
        None => todo!(),
    };
    // println!("{:#?}", ret);
}
