use floem::prelude::*;

struct Sidebar {
    requests: String,
}

pub fn sidebar() -> Stack {
    let new_request = button("New");

    let top_stack = stack((label(move || String::from("History")), new_request))
        .style(move |s| s.flex().width_full())
        .style(move |s| {
            s.justify_between()
                .padding(5)
                .border(1.0)
                .border_color(Color::from_rgb8(205, 205, 205))
        });

    let virtual_stack = VirtualStack::with_view(
        || 0..100,
        move |item| {
            label(move || format!("Item {} with long lines", item)).style(move |s| {
                s.text_ellipsis()
                    .height(22)
                    .padding(10.0)
                    .padding_top(3.0)
                    .padding_bottom(3.0)
                    .width_full()
                    .items_start()
                    .border_bottom(1.0)
                    .border_color(Color::from_rgb8(205, 205, 205))
            })
        },
    )
    .style(move |s| s.flex_col().width_full())
    .scroll()
    .style(move |s| {
        s.border_right(1.0)
            .border_top(1.0)
            .border_color(Color::from_rgb8(205, 205, 205))
    });

    v_stack((top_stack, virtual_stack))
}
