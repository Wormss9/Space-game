use crate::{
    components::{GravityMass, Position},
    state::State,
    vector::Vector,
};

pub fn gravity_system(state: &mut State) {
    let sources: Vec<Option<(Vector, f64)>> = state
        .world
        .query::<&Position>()
        .iter()
        .map(|(source, r_position)| {
            state
                .world
                .query::<(&GravityMass, &Position)>()
                .iter()
                .map(|(receptor, (mass, s_position))| {
                    if source != receptor {
                        let difference = s_position.position - r_position.position;

                        (
                            difference * mass.mass / difference.length() / difference.length_sqr(),
                            difference.length(),
                        )
                    } else {
                        (Vector::new(0.0, 0.0), 0.0)
                    }
                })
                .reduce(|x, y| ((x.0 + y.0), (x.1 + y.1)))
        })
        .collect();

    for ((_, position), acceleration) in state
        .world
        .query_mut::<&mut Position>()
        .into_iter()
        .zip(sources)
    {
        if let Some(acceleration) = acceleration {
            position.acceleration += acceleration.0;
            position.debug = acceleration.1;
        }
    }
}
