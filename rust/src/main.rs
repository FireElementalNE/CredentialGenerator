use anyhow::Result;
use clap::Parser;
use rust_cred_gen::{alphabet, username};
use rust_cred_gen::args::Cli;

fn main() -> Result<()> {
    let cli = Cli::parse();
    if cli.length >= 8 {
        let mut rng = urandom::new();
        let mut uname = username::Username::new(
            rng.next_u64(),
        )?;
        uname.make_username()?;
        println!("Username: {}", uname.un);
        let mut alpha = alphabet::Alphabet::new(
            rng.next_u64()
        );
        alpha.make_password(cli.length)?;
        println!("Password: {}", alpha.pw);

    } else {
        println!("Password length must be at least 8, not {}", cli.length);
    }
    Ok(())
}
