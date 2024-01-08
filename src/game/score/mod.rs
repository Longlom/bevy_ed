use bevy::prelude::*;



pub mod resources;
mod systems;

use self::{resources::*, systems::*};

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app
        .init_resource::<Score>()
        .add_systems(Update, update_score)
        
        ;
    }
}