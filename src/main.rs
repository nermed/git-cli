use clap::Parser;
mod init;
mod create_command;
mod status;
mod add_commit;
mod push;
mod option;

#[derive(Parser)]
#[command(author="Don Nermed", version="0.1", about="To minify git command", long_about = None)]
#[command(propagate_version = true)]
struct Cli {
    #[arg(short, long)]
    init: bool,

    #[arg(short, long)]
    add_commit: Option<String>,

    #[arg(short, long)]
    status: bool,

    #[arg(short, long)]
    push: bool,
}

fn main() {
    let cli = Cli::parse();

    // Check if with specified init
    if cli.init {
        init::init();
    } else if cli.status {
        status::status();
    } else if let Some(commit) = cli.add_commit.as_deref() {
        add_commit::add_commit(commit);
    } else if cli.push {
        push::push();
    }
}