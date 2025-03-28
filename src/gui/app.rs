use floem::{
    peniko::Color,
    style::{CustomStylable, CustomStyle},
    views::{
        Decorators,
        resizable::{self, ResizableStack},
    },
};

use super::{center::center, sidebar::sidebar};
pub fn create() -> ResizableStack {
    let dragger_color = Color::from_rgb8(0, 0, 0);
    let active_dragger_color = Color::from_rgb8(60, 60, 60);
    resizable::resizable((sidebar(), center()))
        .style(|s| s.width_full().height_full())
        .custom_style(move |s| {
            s.handle_color(dragger_color).handle_thickness(4)
                .active(|s| s.handle_color(active_dragger_color))
        })
}
