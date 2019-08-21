fn guitar() {
    println!("guitar") ;
}
pub mod instrument {
    pub fn clarinet() {
        println!("clarinet") ;
        super::guitar();
    }
    mod woodwind {
    }
}
mod voice {

}
