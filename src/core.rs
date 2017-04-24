#[derive(Debug)]
pub enum Epiq {
    Unit,
    Int8(i64),
    Text(String),

    Cpiq{ p: Box<Epiq>, q: Box<Epiq> }, // cons cell piq
    /*
    Eexp(Option<Box<Epiq>>, Box<Epiq>),
    */
}
