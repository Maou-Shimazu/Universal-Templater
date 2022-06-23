#![allow(dead_code)]
pub struct Ts {
    pub name: String,
    pub cli: bool, // if cli, main.ts
    pub web: bool, // if web, index.ts
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

// note: add deno flag and npm flag, add a makefile along with it
impl Ts {
    pub fn configure(name: String, cli: bool, web: bool, files: Vec<String>, directories: Vec<String>) -> Ts {
        Ts { name, cli, web, files, directories }
    }

    pub fn command(self, deno: bool, npm: bool) -> Ts {
        return self;
    }
}