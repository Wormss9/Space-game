use crate::{
    components::{Position, Rotation},
    state::State,
    vector::Vector,
};

pub fn movement_system(state: &mut State, time_data: (f64, f64)) {
    for (_, position) in state.world.query_mut::<&mut Position>() {
        position.speed += position.acceleration * time_data.0;
        position.position +=
            position.speed * time_data.0 + position.acceleration * time_data.1 * 0.5;
        position.acceleration = Vector::new(0.0, 0.0)
    }

    for (_, rotation) in state.world.query_mut::<&mut Rotation>() {
        rotation.rotation += rotation.rotation_speed * time_data.0;
    }
}
