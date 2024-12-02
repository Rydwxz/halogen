use vizia::prelude::*;

const APP_TITLE: &'static str = "Halogen";

fn main() -> Result<(), ApplicationError> {
    Application::new(|cx| {
        make_transport(cx);
        Dialog::new(cx, &true, |cx| {
            Label::new(cx, "Dialog Label");
        });
    })
    .title(APP_TITLE)
    .run()
}

fn make_transport(cx: &mut Context) {
    HStack::new(cx, |cx| {
        Button::new(cx, |cx| Label::new(cx, "Home"));
        Button::new(cx, |cx| Label::new(cx, "Prev"));
        Button::new(cx, |cx| Label::new(cx, "Stop"));
        Button::new(cx, |cx| Label::new(cx, "Pause"));
        Button::new(cx, |cx| Label::new(cx, "Play"));
        Button::new(cx, |cx| Label::new(cx, "Next"));
        Button::new(cx, |cx| Label::new(cx, "End"));
    })
    .child_left(Stretch(0.5))
    .child_right(Stretch(0.5))
    .child_bottom(Stretch(1.0));
}
