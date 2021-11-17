use crate::prelude::*;

//START: header
#[system(for_each)]
#[read_component(Player)]
#[read_component(FieldOfView)]
pub fn movement(
    //END: header
    entity: &Entity,
    want_move: &WantsToMove,
    //START_HIGHLIGHT
    #[resource] map: &mut Map,
    //END_HIGHLIGHT
    #[resource] camera: &mut Camera,
    ecs: &mut SubWorld,
    commands: &mut CommandBuffer
) {
    //END: header
    if map.can_enter_tile(want_move.destination) {
        commands.add_component(want_move.entity, want_move.destination);

        //START: body
        if let Ok(entry) = ecs.entry_ref(want_move.entity) {
            if let Ok(fov) = entry.get_component::<FieldOfView>() {
                commands.add_component(want_move.entity, fov.clone_dirty());

                if entry.get_component::<Player>().is_ok()// <callout id="co.wcis.mem.is_player" />
                {
                    camera.on_player_move(want_move.destination);
                    fov.visible_tiles.iter().for_each(|pos| {// <callout id="co.wcis.mem.vis_tiles" />
                        map.revealed_tiles[map_idx(pos.x, pos.y)] = true;
                    });
                }
            }
        }
        //END: body
    }
    commands.remove(*entity);
}
