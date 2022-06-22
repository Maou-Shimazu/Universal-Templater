use crate::ts::Ts;

pub trait ProjectBuilder {
    fn build(self) -> Result<(), ()>;
}
impl ProjectBuilder for Ts {
    fn build(self) -> Result<(), ()> {
        
        Ok(())
    }
}