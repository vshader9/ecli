use clap::Parser;
use ecli::{sign, verify, Cli, Commands, JwtCommand};
fn main() {
    let cli = Cli::parse();
    //println!("{:?}", cli.cmd);
    match cli.cmd {
        Commands::Jwt(subcommand) => {
            match subcommand {
                JwtCommand::Sign(opts) => sign(&opts),
                JwtCommand::Verify(opts) => verify(&opts),
            };
        }
    }
}
