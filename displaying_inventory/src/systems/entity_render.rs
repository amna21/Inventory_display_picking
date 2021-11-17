use crate::prelude::*;

//START: er_header
#[system]
#[read_component(Point)]
#[read_component(Render)]
#[read_component(FieldOfView)]
#[read_component(Player)]
pub fn entity_render(
    #[resource] camera: &Camera,
    ecs: &SubWorld,
) {
    let mut renderables = <(&Point, &Render)>::query();
    // START_HIGHLIGHT
    let mut fov = <&FieldOfView>::query().filter(component::<Player>());
    // END_HIGHLIGHT
    //END: er_header
    let mut draw_batch = DrawBatch::new();
    draw_batch.target(1);
    let offset = Point::new(camera.left_x, camera.top_y);

    //START: er_filter
    let player_fov = fov.iter(ecs).nth(0).unwrap();

    renderables.
        iter(ecs)
        // START_HIGHLIGHT
        .filter(|(pos, _)| player_fov.visible_tiles.contains(&pos))
        // END_HIGHLIGHT
        .for_each(|(pos, render)| {
            draw_batch.set(
                *pos - offset,
                render.color,
                render.glyph
            );
        }
    );
    //END: er_filter

    draw_batch.submit(5000).expect("Batch error");
}
