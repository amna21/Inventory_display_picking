use crate::prelude::*;

//START: tt_header
#[system]
#[read_component(Point)]
#[read_component(Name)]
// START_HIGHLIGHT
#[read_component(FieldOfView)]
#[read_component(Player)]
// END_HIGHLIGHT
pub fn tooltips(
    ecs: &SubWorld,
    #[resource] mouse_pos: &Point,
    #[resource] camera: &Camera
) {
    let mut positions = <(Entity, &Point, &Name)>::query();
    // START_HIGHLIGHT
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    // END_HIGHLIGHT
    //END: tt_header
    let offset = Point::new(camera.left_x, camera.top_y);
    let map_pos = *mouse_pos + offset;
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    //START: tt_filter
    let player_fov = fov.iter(ecs).nth(0).unwrap();
    positions
        .iter(ecs)
        // START_HIGHLIGHT
        .filter(|(_, pos, _)| 
            **pos == map_pos && player_fov.visible_tiles.contains(&pos) 
        )
        //END_HIGHLIGHT
        //END: tt_filter
        .for_each(|(entity, _, name) | {
            let screen_pos = *mouse_pos * 4;
            let display = if let Ok(health) = ecs.entry_ref(*entity)
                .unwrap()
                .get_component::<Health>() 
            {
                format!("{} : {} hp", &name.0, health.current)
            } else {
                name.0.clone()
            };
            draw_batch.print(screen_pos, &display);
        });
    draw_batch.submit(10100).expect("Batch error");
}
