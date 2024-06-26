pub mod build;
pub mod config;
pub mod run;
use build::build;
use config::get_vm_list;
use run::run;
use structopt::StructOpt;

#[derive(Debug, StructOpt)]
enum Command {
    Build,
    Run,
}

#[derive(StructOpt)]
struct RunVM {
    #[structopt(subcommand)] // Note that we mark a field as a subcommand
    command: Command,
}

fn main() {
    let args = RunVM::from_args();
    let vm_list = get_vm_list();

    match args.command {
        Command::Build => {
            build(&vm_list);
        }
        Command::Run => {
            run(&vm_list);
        }
    }

    println!("command = {:?}", args.command);
}
