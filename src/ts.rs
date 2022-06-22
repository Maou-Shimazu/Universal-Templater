pub struct Ts {
    cli: bool,
    web: bool,
    main: Option<String>,
    index: Option<String>,
    files: Vec<String>,
}

impl Ts {
    pub fn new() -> Ts {
        Ts {
            cli: false,
            web: false,
            main: None,
            index: None,
            files: vec!["deps.ts".to_string()],
        }
    }

}

