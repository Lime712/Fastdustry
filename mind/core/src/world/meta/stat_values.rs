//! Utilities for displaying certain stats in a table.
use crate::world::meta::stat::StatValue;
use crate::world::meta::stat_unit::StatUnit;

pub fn number(value: f32, unit: StatUnit) -> StatValue {
    todo!("number"); // todo this part, cause i dont understand anuke's code there
    StatValue {
        value,
        display: "".to_string(),
    }
}