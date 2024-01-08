use bevy::prelude::*;

pub mod enemy;
mod player;
pub mod star;
pub mod score;
mod systems;

use enemy::EnemyPlugin;
use player::PlayerPlugin;
use score::ScorePlugin;
use star::StarPlugin;
use systems::*;

use crate::{events::GameOver, AppState};

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app
        .add_state::<SimulationState>()
        .add_event::<GameOver>()
        .add_plugins((EnemyPlugin, StarPlugin, PlayerPlugin, ScorePlugin))
        .add_systems(Update, toggle_simulation.run_if(in_state(AppState::Game)));
    }
}


#[derive(States, Debug, Clone, Copy, Hash, PartialEq, Eq, Default)]
pub enum SimulationState {
   Running,
   #[default]
   Paused,
}