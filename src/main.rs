use floem::{Application, kurbo::Size, style::Style, views::Decorators, window::WindowConfig};
use paperboy::common::theme;
use paperboy::gui::app::create;
fn main() {
    let theme = Style::new()
        .background(theme::PRIMARY_BG)
        .color(theme::PRIMARY);
    let app = Application::new().window(
        |_| create().style(move |_| theme.clone()),
        Some(
            WindowConfig::default()
                .size(Size::new(1200.0, 900.0))
                .min_size(Size::new(800.0, 600.0))
                .title("Paperboy"),
        ),
    );
    app.run();
}
