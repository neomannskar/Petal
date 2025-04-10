use rand::Rng;

pub struct Identifier {
    pub name: String,
    pub id: usize,
}

impl Identifier {
    fn new(name: String) -> Self {
        let mut rng = rand::rng();
        let id = rng.random_range(0..=usize::MAX);
        Identifier { name, id }
    }
}

impl From<String> for Identifier {
    fn from(name: String) -> Self {
        Identifier::new(name)
    }
}

impl From<&str> for Identifier {
    fn from(name: &str) -> Self {
        Identifier::new(name.to_string())
    }
}
