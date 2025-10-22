use bevy::prelude::*;
use rand::Rng;
use super::resources::*;
use super::events::*;

pub struct EnvironmentPlugin;

impl Plugin for EnvironmentPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, update_environment.run_if(in_state(GameState::InGame)));
    }
}

fn update_environment(
    mut environment: ResMut<Environment>,
    time: Res<Time>,
    mut events: ResMut<RandomEvents>,
    mut event_writer: EventWriter<TriggerEvent>,
) {
    environment.time_of_day = (environment.time_of_day + time.delta_seconds() / 86400.0) % 1.0;
    let mut rng = rand::thread_rng();
    if rng.gen_bool(0.005) {
        let event = events.events[rng.gen_range(0..events.events.len())].clone();
        environment.weather = match event.effect {
            EventEffect::WeatherChange(weather) => weather,
            _ => environment.weather,
        };
        event_writer.send(TriggerEvent { event });
    }
}
