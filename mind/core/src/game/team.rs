use std::fmt::format;
use rand::Rng;
use arc::arc_core::graphics::{color, pal};
use arc::arc_core::graphics::color::Color;

// todo: maybe fixing, cause idk if those const functions compile

/// The 6 base teams used in the editor.
pub const BASE_TEAMS: [Team; 6] = gen_base_teams();
/// All 256 registered teams.
pub const ALL: [Team; 256] = gen_teams();

#[derive(Clone, Copy, Debug, PartialEq, Eq, Hash)]
pub struct Team {
    pub id: usize,
    pub color: Color,
    pub palette: Vec<Color>,
    pub paletti: [i32; 3],
    pub emoji: String,
    pub has_palette: bool,
    pub name: &'static str,
}

impl Team {
    pub const fn new(id: usize, name: &'static str, color: Color) -> Self {
        let mut t = Self {
            id,
            name,
            color,
            palette: Vec::new(),
            paletti: [0; 3],
            emoji: String::new(),
            has_palette: false,
        };
        t.palette.push(color.clone());
        t.palette.push(color.clone().mul_float(0.75));
        t.palette.push(color.clone().mul_float(0.5));

        let mut i = 0;
        loop { // hack to make this a const fn
            if i == 3 {
                break;
            }
            t.paletti[i] = t.palette[i].self_rgba8888();
            i += 1;
        }
        t
    }

    pub const fn new_palette(id: usize, name: &'static str, color: Color, palette: Vec<Color>) -> Self {
        let mut t = Self {
            id,
            name,
            color,
            palette,
            paletti: [0; 3],
            emoji: String::new(),
            has_palette: true,
        };
        let mut i = 0;
        loop { // hack to make this a const fn
            if i == 3 {
                break;
            }
            t.paletti[i] = t.palette[i].self_rgba8888();
            i += 1;
        }
        t
    }

    // todo: implement all the other methods, cause rn we dont have eg CoreBuild
}


const fn gen_teams() -> [Team; 256] {
    let mut rng = rand::thread_rng();
    let mut teams = [Team::new(0, "sharded", color::WHITE); 256];
    let mut i = 0;
    loop { // hack to make this a const fn
        if i < 6 {
            teams[i] = BASE_TEAMS[i].clone();
            i += 1;
            continue;
        }
        if i == 256 {
            break;
        }
        let name = &*format!("team#{}", i);
        let color = Color::hsv_to_rgb(rng.gen_range(0, 360), rng.gen_range(0, 100), rng.gen_range(0, 100));
        teams[i] = Team::new(i, name, color);
        i += 1;
    }
    teams
}

const fn gen_base_teams() -> [Team; 6] {
    let mut teams = [Team::new(0, "derelict", Color::WHITE); 6];
    teams[0] = Team::new(0, "derelict", Color::value_of("4d4e58"));
    teams[1] = Team::new_palette(1, "sharded", pal::ACCENT.clone(), vec![color::value_of("ffd37f"), color::value_of("eab678"), color::value_of("d4816b")]);
    teams[2] = Team::new_palette(2, "crux", color::value_of("f25555"), vec![color::value_of("fc8e6c"), color::value_of("f25555"), color::value_of("a04553")]);
    teams[3] = Team::new_palette(3, "malis", color::value_of("a27ce5"), vec![color::value_of("c195fb"), color::value_of("665c9f"), color::value_of("484988")]);

    // TODO: temporarily no palettes for these teams.
    teams[4] = Team::new(4, "green", color::value_of("54d67d"));
    teams[5] = Team::new(5, "blue", color::value_of("6c87fd"));
    teams[6] = Team::new(6, "neoplastic", color::value_of("e05438"));// yes, it looks very similar to crux, you're not supposed to use this team for block regions anyway
    teams
}