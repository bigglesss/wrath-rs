mod tutorial_flags;
pub use tutorial_flags::TutorialFlags;

mod action_bar;
pub use action_bar::ActionBar;

mod data_storage;
pub use data_storage::*;

mod character_inventory;
pub use character_inventory::*;

use wow_world_messages::wrath::{Area, Map, Vector3d};
#[derive(Debug, Clone, Default, PartialEq)]
pub struct WorldZoneLocation {
    pub map: Map,
    pub area: Area,
    pub position: Vector3d,
    pub orientation: f32,
}

#[derive(Debug, Clone, Default, PartialEq)]
pub struct PositionAndOrientation {
    pub position: Vector3d,
    pub orientation: f32,
}

impl From<WorldZoneLocation> for PositionAndOrientation {
    fn from(wzl: WorldZoneLocation) -> Self {
        Self {
            position: wzl.position,
            orientation: wzl.orientation,
        }
    }
}
