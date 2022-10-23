use clap::Parser;
use git_explore::*;

fn main() {
    let cli = RepoCli::parse();
    match cli.command {
        Some(Command::List(opt)) => run_list(&opt).unwrap(),
        Some(Command::Init(opt)) => init(&opt).unwrap(),
        Some(Command::Commit(mut opt)) => commit(&mut opt).unwrap(),
        Some(Command::Pull(opt)) => pull(&opt).unwrap(),
        Some(Command::Push(opt)) => push(&opt).unwrap(),
        None => todo!(),
    };
}
