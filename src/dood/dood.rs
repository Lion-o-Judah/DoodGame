use rand::Rng;
use std::fmt;

pub struct Dood {
    pub name: String,
    pub position: (f32,f32),
    pub sex: Sex,
    pub satiation: f32,
    pub fat_stores: f32,
    pub muscle_mass: f32,
    pub basal_metobolic_rate: f32,
}

pub enum Sex {
    Male,
    Female
}

impl fmt::Display for Sex {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        match *self {
            Sex::Male => write!(f, "Male"),
            Sex::Female => write!(f, "Female"),
        }
    }
}

impl Dood {
    pub fn new() -> Self{
        let mut rng = rand::rng();
        let random_sex = if rng.random() {Sex::Male} else {Sex::Female};
        Self {
            name: "test".to_string(),
            position: (0.0,0.0),
            sex: random_sex
        }
    pub fn tick() -> Result<(), DoodError>

    }
}
