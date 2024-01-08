use bevy::prelude::*;

use super::SimulationState;

pub fn toggle_simulation(
    mut commmands: Commands,
    keyboard_input: Res<Input<KeyCode>>,
    simulation_state: Res<State<SimulationState>>,
) {
    if keyboard_input.just_pressed(KeyCode::Space) {
        match simulation_state.get() {
            SimulationState::Paused => {
                commmands.insert_resource(NextState(Some(SimulationState::Running)));
                println!("Simulatuin Running");
            }
            SimulationState::Running => {
                commmands.insert_resource(NextState(Some(SimulationState::Paused)));
                println!("Simulatuin Paused");
            }
        }
  
    }
}
