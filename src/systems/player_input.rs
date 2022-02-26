use crate::prelude::*;

#[system]
#[write_component(Point)]
#[read_component(Player)]
pub fn player_input(
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key: &Option<VirtualKeyCode>,
    #[resource] turn_state: &mut TurnState,
) {
    let mut players = <(Entity, &Point)>::query()
        .filter(component::<Player>());
    if let Some(key) = key {
        let delta = match key{
            VirtualKeyCode::A => Point::new(-1, 0),
            VirtualKeyCode::D => Point::new(1, 0),
            VirtualKeyCode::W => Point::new(0, -1),
            VirtualKeyCode::S => Point::new(0, 1),
            _ => Point::new(0, 0),
        };
        players.iter(ecs).for_each(|(entity, pos)| {
            let destination = *pos + delta;
            commands
                .push(((), WantsToMove { entity: *entity, destination }));
        });
        *turn_state = TurnState::PlayerTurn;
    }
}