pub mod args {
    use clap::Parser;
    #[derive(Parser)]
    #[clap(author, version, about, long_about = "Generate a random username and password.")]
    pub struct Cli {
        /// Length of the password
        #[clap(value_parser)]
        pub length: u32,
    }
}

pub mod resources {
    use std::ops::Deref;
    use rust_embed::RustEmbed;
    use anyhow::{Result, Context};
    pub const ADJ_FILE: &str = "adjectives";
    pub const NOUN_FILE: &str = "nouns";
    #[derive(RustEmbed)]
    #[folder = "../res/"]
    pub struct Asset;
    fn get_file(filename: &str) -> Result<Vec<String>> {
        let err_msg = format!("Could not get font {filename}");
        let raw: rust_embed::EmbeddedFile = Asset::get(filename).context(err_msg)?;
        let a =
            raw.data
                .deref()
                .to_vec()
                .iter()
                .map(|e| {
                    *e as char
                }).collect::<Vec<char>>()
                .into_iter()
                .collect::<String>()
                .split("\n")
                .map(str::to_string)
                .collect::<Vec<String>>();
        Ok(a)
    }
    pub fn get_nouns() -> Result<Vec<String>> {
        Ok(get_file(NOUN_FILE)?)
    }
    pub fn get_adjs() -> Result<Vec<String>> {
        Ok(get_file(ADJ_FILE)?)
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
    use anyhow::{Context, Result};
    use crate::resources::{get_adjs, get_nouns};
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
    pub struct Username {
        ig: IndexGen,
        nouns: Vec<String>,
        adjs: Vec<String>,
        pub un: String,
    }
    impl Username {
        pub fn new(seed: u64) -> Result<Username> {
            let un = Username {
                ig: IndexGen::new(seed),
                nouns: get_nouns()?,
                adjs: get_adjs()?,
                un: "".to_string()
            };
            Ok(un)
        }

        pub fn make_username(&mut self) -> Result<()> {
            let noun = title_case(
                self.nouns.get(
                    self.ig.gen_index::<usize>(self.nouns.len() as u32)?
                )
                    .context(index_err("nouns"))?.to_string())?;
            let adj  = title_case(
                self.adjs.get(
                    self.ig.gen_index::<usize>(self.adjs.len() as u32)?
                )
                    .context(index_err("adjs"))?.to_string())?;
            self.un = adj + noun.as_str();
            Ok(())
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
        pub pw: String,

    }
    impl Alphabet {
        pub fn new(seed: u64) -> Alphabet {
            Alphabet {
                ig: IndexGen::new(seed),
                pw: "".to_string()
            }
        }
        fn create_pw(&mut self, length: u32) -> Result<String> {
            let mut out_str: String = String::new();
            for _ in 0..length {
                let index = self.ig.gen_index::<usize>(VALID_CHARS.len() as u32)?;
                let my_char = ((
                    *VALID_CHARS.get(index)
                        .context(index_err("VALID_CHARS"))? as u8
                ) as char).to_string();
                out_str.push_str(&my_char)
            }
            Ok(out_str)
        }
        fn check_pw_reqs(pw: &str) -> bool {
            pw.chars()
                .map(|c|c.is_numeric())
                .fold(false, |x, y| y || x) &&
                pw.chars()
                    .map(|c|c.is_uppercase())
                    .fold(false, |x, y| y || x) &&
                pw.chars()
                    .map(|c|c.is_lowercase())
                    .fold(false, |x, y| y || x) &&
                pw.chars()
                    .map(|c| !c.is_alphanumeric())
                    .fold(false, |x, y| y || x)
        }
        pub fn make_password(&mut self, length: u32) -> Result<()> {
            self.pw = self.create_pw(length)?;
            while ! (Self::check_pw_reqs(self.pw.as_str())) {
                self.pw = self.create_pw(length)?;
            }
            Ok(())
        }
    }
}