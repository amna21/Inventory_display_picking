use crate::prelude::*;

#[system]
#[read_component(Point)]
#[write_component(FieldOfView)]
pub fn fov(
    ecs: &mut SubWorld,
    #[resource] map: &Map,
) {
    let mut views = <(&Point, &mut FieldOfView)>::query();// <callout id="co.wcis.fov.fovquery" />
    views
        .iter_mut(ecs)// <callout id="co.wcis.fov.iter_mut" />
        .filter(|(_, fov)| fov.is_dirty)// <callout id="co.wcis.fov.dirty_filter" />
        .for_each(|(pos, mut fov)| {
            fov.visible_tiles = field_of_view_set(*pos, fov.radius, map);// <callout id="co.wcis.fov.call_fov" />
            fov.is_dirty = false;// <callout id="co.wcis.fov.not_dirty" />
        }
    );
}
