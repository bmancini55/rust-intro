mod gui;
use gui::Draw;

fn main() {
    let screen = gui::Screen {
        components: vec![
            Box::new(SelectBox {
                width: 200,
                height: 50,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
            Box::new(gui::Button {
                width: 50,
                height: 20,
                label: String::from("OK"),
            }),
        ],
    };

    screen.run();
}

/// Example of an external struct that implements the draw behavior that
/// is defined in the gui module.
pub struct SelectBox {
    pub width: u32,
    pub height: u32,
    pub options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self) {
        // todo
    }
}
