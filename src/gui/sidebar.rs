use super::components::tabs;
use floem::{peniko::color, prelude::*};

struct Sidebar {
    requests: String,
}

pub fn sidebar() -> Stack {
    let new_request = button("New");

    let top_bar = stack((
        label(move || String::from("History")).style(move |s| s.padding(5)),
        new_request,
    ))
    .style(move |s| s.flex().width_full())
    .style(move |s| s.justify_between().padding(5));

    v_stack((top_bar, tabs::draggable_view())).style(move |s| s.height_full())
}
