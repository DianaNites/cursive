use cursive::theme::{BaseColor, BorderStyle, Color, ColorStyle};
use cursive::views::{Dialog, Edit, LinearLayout, Text};
use cursive::Cursive;

fn main() {
    let mut siv = Cursive::default();

    let layout = LinearLayout::vertical()
        .child(Text::new("This is a dynamic theme example!"))
        .child(Edit::new().content("Woo! colors!").style(ColorStyle::new(
            Color::Rgb(200, 150, 150),
            Color::Dark(BaseColor::Blue),
        )));

    siv.add_layer(
        Dialog::around(layout)
            .button("Change", |s| {
                let mut theme = s.current_theme().clone();

                theme.shadow = !theme.shadow;
                theme.borders = match theme.borders {
                    BorderStyle::Simple => BorderStyle::Outset,
                    BorderStyle::Outset => BorderStyle::None,
                    BorderStyle::None => BorderStyle::Simple,
                };

                s.set_theme(theme);
            })
            .button("Quit", Cursive::quit),
    );

    siv.run();
}
