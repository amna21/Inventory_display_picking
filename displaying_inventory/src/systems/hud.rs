use crate::prelude::*;

//START: headers
#[system]
#[read_component(Health)]
#[read_component(Player)]
// START_HIGHLIGHT
#[read_component(Item)]
#[read_component(Carried)]
#[read_component(Name)]
// END_HIGHLIGHT
pub fn hud(ecs: &SubWorld) {
    //END: headers
    let mut health_query = <&Health>::query().filter(component::<Player>());
    let player_health = health_query
        .iter(ecs)
        .nth(0)
        .unwrap();

    let mut draw_batch = DrawBatch::new();
    draw_batch.target(2);
    draw_batch.print_centered(1, 
        "Explore the Dungeon. Cursor keys to move.");
    draw_batch.bar_horizontal(
        Point::zero(),
        SCREEN_WIDTH*2,
        player_health.current,
        player_health.max,
        ColorPair::new(RED, BLACK)
    );
    draw_batch.print_color_centered(
        0,
        format!(" Health: {} / {} ", 
            player_health.current, 
            player_health.max
        ),
        ColorPair::new(WHITE, RED)
    );

    //START: hud_items
    let player = <(Entity, &Player)>::query()
                    .iter(ecs)
                    .find_map(|(entity, _player)| Some(*entity))
                    .unwrap();// <callout id="co.inventory.hud.find_the_player" />
    let mut item_query = <(&Item, &Name, &Carried)>::query();// <callout id="co.inventory.carry_query" />
    let mut y = 3;// <callout id="co.inventory.muty" />
    item_query
        .iter(ecs)
        .filter(|(_, _, carried)| carried.0 == player)// <callout id="co.inventory.item_filter_p" />
        .for_each(|(_, name, _)| {
            draw_batch.print(// <callout id="co.inventory.item_print" />
                Point::new(3, y), 
                format!("{} : {}", y-2, &name.0)
            );
            y += 1;
        }
    );
    if y > 3 {// <callout id="co.inventory.item_heading" />
        draw_batch.print_color(Point::new(3, 2), "Items carried", 
            ColorPair::new(YELLOW, BLACK)
        );
    }
    //END: hud_items

    draw_batch.submit(10000).expect("Batch error");
}
