use floem::prelude::*;

use crate::common::theme::SECONDARY_BG;

fn sortable_item(
    name: &str,
    sortable_items: RwSignal<Vec<usize>>,
    dragger_id: RwSignal<usize>,
    item_id: usize,
) -> impl IntoView {
    let name = String::from(name);
    stack((
        label(move || format!("Selectable item {name}"))
            .style(|s| s.padding(5).width_full().selectable(false)),
        "drag me".style(|s| s.selectable(false).padding(2)),
    ))
    .draggable()
    .on_event_stop(floem::event::EventListener::DragStart, move |_| {
        dragger_id.set(item_id);
    })
    .on_event_stop(floem::event::EventListener::DragOver, move |_| {
        if dragger_id.get_untracked() != item_id {
            let dragger_pos = sortable_items
                .get()
                .iter()
                .position(|id| *id == dragger_id.get_untracked())
                .unwrap();
            let hover_pos = sortable_items
                .get()
                .iter()
                .position(|id| *id == item_id)
                .unwrap();

            sortable_items.update(|items| {
                items.remove(dragger_pos);
                items.insert(hover_pos, dragger_id.get_untracked());
            });
        }
    })
    .dragging_style(|s| {
        s.box_shadow_blur(3)
            .box_shadow_color(Color::from_rgb8(100, 100, 100))
            .box_shadow_spread(2)
    })
    .style(move |s| s.selectable(false).col_gap(10).padding(20).items_center().hover(move |s| s.background(SECONDARY_BG)))
}

pub fn draggable_view() -> DynStack<usize> {
    let items = [
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine", "ten",
    ];
    let sortable_items = RwSignal::new((0..items.len()).collect::<Vec<usize>>());
    let dragger_id = RwSignal::new(0);

    dyn_stack(
        move || sortable_items.get(),
        move |item_id| *item_id,
        move |item_id| sortable_item(items[item_id], sortable_items, dragger_id, item_id),
    )
    .style(|s| s.flex_col().row_gap(5).padding(10))
}
