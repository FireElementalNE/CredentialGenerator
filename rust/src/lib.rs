pub mod args {
    use std::str::FromStr;
    use anyhow::Result;
    use clap::{Arg, ArgMatches, Command};
    pub const LEN_ARG: &str = "length";
    pub const NOUN_ARG: &str = "nouns";
    pub const ADJ_ARG: &str = "adjectives";
    fn get_args() -> ArgMatches {
        let app = Command::new("Credential Gen")
            .version(env!("CARGO_PKG_VERSION"))
            .about("Generate a random username and password")
            .arg(Arg::new(LEN_ARG)
                     .required(true)
                     .help("Length of the password"),
            )
            .arg(Arg::new(NOUN_ARG)
                 .required(true)
                 .help("A text file containing nouns (newline delimited)"),
            )
            .arg(Arg::new(ADJ_ARG)
                .required(true)
                .help("A text file containing adjectives (newline delimited)"),
            );
        app.get_matches()
    }
    pub struct AppArgs {
        am: ArgMatches
    }
    impl AppArgs {
        pub fn new() -> Self {
            Self {
                am: get_args(),
            }
        }
        pub fn get_argument<T>(&self, arg_name: &str) -> Result<T, T::Err> where T: FromStr {
            let arg = self.am
                .value_of(arg_name)
                .expect(format!("Argument {arg_name} not present").as_str())
                .parse::<T>()?;
            Ok(arg)
        }
    }
}

pub mod utils {
    use std::str::FromStr;
    use rand::Rng;
    use rand_chacha::ChaCha20Rng;
    use rand_chacha::rand_core::SeedableRng;

    pub struct IndexGen {
        rng: ChaCha20Rng,
    }

    impl IndexGen {
        pub fn new(seed: u64) -> IndexGen{
            IndexGen {
                rng: ChaCha20Rng::seed_from_u64(seed)
            }
        }
        pub fn gen_index<T>(&mut self, max_len: u32) -> Result<T, T::Err> where T: FromStr {
            let a = self.rng.gen::<u32>() % max_len;
            Ok(a.to_string().parse::<T>()?)
        }
    }

    pub fn index_err(name: &str) -> String {
        format!("Could not fetch index from {}", name)
    }

}

pub mod username {
    use std::fs::File;
    use std::io::{BufRead, BufReader};
    use anyhow::{Context, Result};
    use crate::utils::{index_err, IndexGen};

    pub enum SizeType {
        NounSize,
        AdjSize
    }
    fn title_case(n: String) -> Result<String> {
        let mut arr = n
            .chars()
            .filter(|x| *x != ' ')
            .collect::<Vec<char>>();
        let mut first = true;
        for el in arr.iter_mut() {
            match first {
                true => {
                    *el = el
                        .to_uppercase()
                        .to_string()
                        .parse::<char>()?;
                    first = false;
                }
                false => {}
            }
        }
        Ok(String::from_iter(arr))
    }
    fn lines_from_file(filename: String) -> Result<Vec<String>> {
        let file: File = File::open(filename)?;
        let buf: BufReader<File> = BufReader::new(file);
        let res = buf.lines().filter_map(|line| {
            match line {
                Ok(l) => {
                    Some(l)
                }
                Err(_) => {
                    None
                }
            }
        }).collect::<Vec<String>>();
        Ok(res)
    }
    pub struct Username {
        ig: IndexGen,
        nouns: Vec<String>,
        adjs: Vec<String>,
    }
    impl Username {
        pub fn new(seed: u64, nfile: String, afile: String) -> Result<Username> {
            let un = Username {
                ig: IndexGen::new(seed),
                nouns: lines_from_file(nfile)?,
                adjs: lines_from_file(afile)?
            };
            Ok(un)
        }

        pub fn make_username(&mut self) -> Result<String> {
            let noun = title_case(
                self.nouns.get(
                    self.ig.gen_index::<usize>(self.nouns.len() as u32)?
                )
                .context(index_err("nouns"))?.to_string())?;
            let mut adj  = title_case(
                self.adjs.get(
                self.ig.gen_index::<usize>(self.adjs.len() as u32)?
                )
                .context(index_err("adjs"))?.to_string())?;
            adj.push_str(&noun);
            Ok(adj.to_string())
        }
    }
}

pub mod alphabet {
    use anyhow::{Context, Result};
    use crate::utils::{index_err, IndexGen};

    const VALID_CHARS: [u8; 82] =
        [48, 49, 50, 51, 52, 53, 54, 55, 56,
        57, 33, 34, 35, 36, 37, 38, 40, 41, 42, 43, 44, 45, 46, 58, 59, 60, 61,
        62, 63, 64, 65, 66, 67, 68, 69, 70, 71, 72, 73, 74, 75, 76, 77, 78, 79,
        80, 81, 82, 83, 84, 85, 86, 87, 88, 89, 90, 97, 98, 99, 100, 101, 102,
        103, 104, 105, 106, 107, 108, 109, 110, 111, 112, 113, 114, 115, 116,
        117, 118, 119, 120, 121, 122];

    pub struct Alphabet {
        ig: IndexGen,

    }
    impl Alphabet {
        pub fn new(seed: u64) -> Alphabet {
            Alphabet {
                ig: IndexGen::new(seed),
            }
        }
        pub fn make_password(&mut self, length: u32) -> Result<String> {
            let mut outstr: String = String::new();
            for _ in 0..length {
                let index = self.ig.gen_index::<usize>(VALID_CHARS.len() as u32)?;
                let my_char = ((
                    *VALID_CHARS.get(index)
                        .context(index_err("VALID_CHARS"))? as u8
                ) as char).to_string();
                outstr.push_str(&my_char)
            }
            Ok(outstr)
        }
    }
}