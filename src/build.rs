use crate::structs::*;

pub trait ProjectBuilder {
    fn build(self) -> Result<(), ()>;
}

impl ProjectBuilder for Ts {
    fn build(self) -> Result<(), ()> {
        std::fs::create_dir_all(&self.name).unwrap();
        std::env::set_current_dir(&self.name).unwrap();
        for i in self.files {
            std::fs::File::create(i).unwrap();
        }
        for i in self.directories {
            std::fs::create_dir(i).unwrap();
        }
        // std::process::Command::new("npm.cmd").spawn().unwrap(); 
        // note: npm is not an executable, replicate script to spawn npm
        Ok(())
    }
    
}
