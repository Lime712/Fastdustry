pub enum Category {
    /// Offensive turrets. 
    Turret,
    /// Blocks that produce raw resources, such as drills. 
    Production,
    /// Blocks that move items around. 
    Distribution,
    /// Blocks that move liquids around. 
    Liquid,
    /// Blocks that generate or transport power. 
    Power,
    /// Walls and other defensive structures. 
    Defense,
    /// Blocks that craft things. 
    Crafting,
    /// Blocks that create units. 
    Units,
    /// Things for storage or passive effects. 
    Effect,
    /// Blocks related to logic. 
    Logic,
}

impl Category {
    pub fn index(&self) -> u8 {
        match self {
            Category::Turret => 0,
            Category::Production => 1,
            Category::Distribution => 2,
            Category::Liquid => 3,
            Category::Power => 4,
            Category::Defense => 5,
            Category::Crafting => 6,
            Category::Units => 7,
            Category::Effect => 8,
            Category::Logic => 9,
        }
    }

    pub fn get(index: u8) -> Category {
        match index {
            0 => Category::Turret,
            1 => Category::Production,
            2 => Category::Distribution,
            3 => Category::Liquid,
            4 => Category::Power,
            5 => Category::Defense,
            6 => Category::Crafting,
            7 => Category::Units,
            8 => Category::Effect,
            9 => Category::Logic,
            _ => panic!("Invalid category index: {}", index),
        }
    }

    pub fn prev(&self) -> Category {
        let index =  (self.index() + 9) % 10;
        Category::get(index)
    }

    pub fn next(&self) -> Category {
        let index =  (self.index() + 1) % 10;
        Category::get(index)
    }
}