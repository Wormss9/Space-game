use crate::{
    components::{Position, Rotation},
    state::State,
    vector::Vector,
};

pub fn movement_system(state: &mut State, time_data: (f64, f64)) {
    for (e, position) in state.world.query_mut::<&mut Position>() {
        if e.to_bits().get() == 4294967296 {
            let ke = 0.5 * position.speed.length_sqr();
            let pe = 30.0 - position.acceleration.length() * position.debug;
            println!("ke: {:?}", ke);
            println!("pe:{:?}", pe);
            println!("te: {:?}", ke + pe);
        }
        position.speed += position.acceleration * time_data.0;
        position.position +=
            position.speed * time_data.0 + position.acceleration * time_data.1 * 0.5;
        position.acceleration = Vector::new(0.0, 0.0)
    }

    for (_, rotation) in state.world.query_mut::<&mut Rotation>() {
        rotation.rotation += rotation.rotation_speed * time_data.0;
    }
}
