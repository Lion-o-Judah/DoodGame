use rand::Rng;
use std::fmt;

pub struct Dood {
    pub name: String,
    pub position: (u32,u32),//position on the map
    pub sex: Sex,//gender
    pub satiation: f32,//how satisfied one is
    pub fat_stores: f32, //how many pounds of fat one has
    pub muscle_mass: f32, //how many pounds of muscle one has
    pub basal_metobolic_rate: f32, //how many calories one need every day to live
    pub height: f32, //in inches
    pub natrual_path_finding: f32, //the inate abbility of a dood to find the shortest path to a location ranges from 1% to 100% efficiency
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
//TODO calculate the basal_metobolic_rate based on the doods height and wheight using the
//equations in this article: https://my.clevelandclinic.org/health/body/basal-metabolic-rate-bmr
//TODO add path finding and sources of food
//TODO make doods burn calories over time based off exertion
impl Dood {
    pub fn new() -> Self{ //creates a new dood with no parents basicly an adam or an eve stats are randomly asigned
        let mut rng = rand::rng();
        let random_sex = if rng.random() {Sex::Male} else {Sex::Female};
        let height = rng.random_range(60..=80);
        let basal_metobolic_rate = rng.random_range(1600..=1800);
        Self {
            name: "test".to_string(),
            position: (0.0,0.0),
            sex: random_sex
        }
    pub fn tick() -> Result<(), DoodError>{ //called every tick causes the dood to do all the dood things    
        
    }
}
