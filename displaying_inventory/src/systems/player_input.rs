use crate::prelude::*;

//START: header
#[system]
#[read_component(Point)]
#[read_component(Player)]
#[read_component(Enemy)]
#[write_component(Health)]
//START_HIGHLIGHT
#[read_component(Item)]
#[read_component(Carried)]
//END_HIGHLIGHT
pub fn player_input(
    //END: header
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer,
    #[resource] key : &Option<VirtualKeyCode>,
    #[resource] turn_state : &mut TurnState
) {
    let mut players = <(Entity, &Point)>::query().filter(component::<Player>());
    let mut enemies = <(Entity, &Point)>::query().filter(component::<Enemy>());

    if let Some(key) = *key {
        let delta = match key {
            VirtualKeyCode::Left => Point::new(-1, 0),
            VirtualKeyCode::Right => Point::new(1, 0),
            VirtualKeyCode::Up => Point::new(0, -1),
            VirtualKeyCode::Down => Point::new(0, 1),
            //START: get
            VirtualKeyCode::G => {// <callout id="co.inventory.match_g" />
                let (player, player_pos) = players// <callout id="co.inventory.players_destructure" />
                    .iter(ecs)
                    .find_map(|(entity, pos)| Some((*entity, *pos)))// <callout id="co.inventory.player_find_map" />
                    .unwrap();

                let mut items = <(Entity, &Item, &Point)>::query();// <callout id="co.inventory.item_query" />
                items.iter(ecs)
                    .filter(|(_entity, _item, &item_pos)| item_pos == player_pos)// <callout id="co.inventory.item_filter" />
                    .for_each(|(entity, _item, _item_pos)| {
                        commands.remove_component::<Point>(*entity);// <callout id="co.inventory.no_point" />
                        commands.add_component(*entity, Carried(player));// <callout id="co.inventory.add_carry" />
                    }
                );
                Point::new(0, 0)
            },
            //END: get
            _ => Point::new(0, 0),
        };

        let (player_entity, destination) = players
                .iter(ecs)
                .find_map(|(entity, pos)| Some((*entity, *pos + delta)) )
                .unwrap();

        let mut did_something = false;
        if delta.x !=0 || delta.y != 0 {

        let mut hit_something = false;
        enemies
            .iter(ecs)
            .filter(|(_, pos)| {
                **pos == destination
            })
            .for_each(|(entity, _) | {
                hit_something = true;
                did_something = true;

                commands
                    .push(((), WantsToAttack{
                        attacker: player_entity,
                        victim: *entity,
                    }));
            });

            if !hit_something {
                did_something = true;
                commands
                    .push(((), WantsToMove{
                        entity: player_entity,
                        destination
                    }));
            }
        };
        *turn_state = TurnState::PlayerTurn;
    }
}
