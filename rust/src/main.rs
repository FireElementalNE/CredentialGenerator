use anyhow::Result;
use rust_cred_gen::{alphabet, username};
use rust_cred_gen::args::{AppArgs, LEN_ARG};

fn main() -> Result<()> {
    let args = AppArgs::new();
    let mut rng = urandom::new();
    println!("Username: {}",
             username::Username::new(
                 rng.next_u64(),
             )?.make_username()?
    );
    println!("Password: {}",
             alphabet::Alphabet::new(
                 rng.next_u64()
             ).make_password(args.get_argument::<u32>(LEN_ARG)?)?
    );
    Ok(())
}
