
pub mod instrument {
    pub fn clarinet() {
        // Function body code goes here
        //so we can use super to go to the parent module of instrument
        super::breathe_in();
    }
}
fn breathe_in() {
    println!("callled breathe_in");
}
