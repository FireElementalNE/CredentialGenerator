use anyhow::Result;
use rust_cred_gen::{alphabet, username};
use rust_cred_gen::args::{ADJ_ARG, AppArgs, LEN_ARG, NOUN_ARG};

fn main() -> Result<()> {
    let args = AppArgs::new();
    let mut rng = urandom::new();
    println!("Username: {}",
             username::Username::new(
                 rng.next_u64(),
                 args.get_argument::<String>(NOUN_ARG)?,
                 args.get_argument::<String>(ADJ_ARG)?
             )?.make_username()?
    );
    println!("Password: {}",
             alphabet::Alphabet::new(
                 rng.next_u64()
             ).make_password(args.get_argument::<u32>(LEN_ARG)?)?
    );
    Ok(())
}
