pub struct Ts {
    pub name: String,
    pub files: Vec<String>,
    pub directories: Vec<String>,
}

// note: add deno flag and npm flag, add a makefile along with it
impl Ts {
    pub fn new(name: String) -> Ts {
        Ts {
            name,
            files: vec![],
            directories: vec!["ts".to_string()],
        }
    }

    pub fn command(mut self, deno: bool, npm: bool) -> Ts {
        if deno {
            self.files = vec![
                "deps.ts".to_string(),
                "main.ts".to_string(),
                "makefile".to_string(),
            ];
            // makefile
        } else {
            self.files = vec![
                "index.ts".to_string(),
                "deps.ts".to_string(),
                "tsconfig.json".to_string(),
            ];   
        }
        return self;
    }
}
