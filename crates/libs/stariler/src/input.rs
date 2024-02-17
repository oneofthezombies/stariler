#[derive(Debug)]
pub enum Kind {
    Files(Vec<String>),
    Project(String),
}

#[derive(Debug)]
pub struct Input {
    pub kind: Kind,
}
