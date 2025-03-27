use floem::{peniko::color, prelude::*};

struct Sidebar {
    requests: String,
}

pub fn sidebar() -> Stack {
    let new_request = button("New");

    let top_stack = stack((label(move || String::from("History")).style(move |s| s.padding(5)), new_request))
        .style(move |s| s.flex().width_full())
        .style(move |s| {
            s.justify_between()
                .padding(5)
                .border_bottom(1.0)
                
        });

    let virtual_stack = VirtualStack::with_view(
        || 0..10,
        move |item| {
            label(move || format!("Item {} with long lines", item)).style(move |s| {
                s.text_ellipsis()
                    .height(22)
                    .padding(10.0)
                    .padding_vert(20)
                    .width_full()
                    .items_start().hover(move |s| s.background(color::AlphaColor::from_rgb8(80, 80, 80)))
            })
        },
    )
    .style(move |s| s.flex_col().width_full())
    .scroll()
    .style(move |s| {
        s.border_top(1.0)
    });

    v_stack((top_stack, virtual_stack)).style(move |s| s.height_full().border(1))
}
