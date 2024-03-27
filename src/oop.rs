use learn::Draw;

pub struct Screen {
    pub components: Vec<Box<dyn Draw>>,
}

impl Screen {
    pub fn run(&self) {
        for component in self.components.iter() {
            component.draw();
        }
    }
}

pub struct Button {
    pub width: u32,
    pub height: u32,
    pub label: String,
}

impl Draw for Button {
    fn draw(&self) {
        //code to actually draw a button
        println!("Draw Button");
    }
}

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}
impl Draw for SelectBox {
    fn draw(&self) {
        println!("Draw SelectBox.");
    }
}
fn main() {
    let v = Vec::<Box<dyn Draw>>::new();
    let screen = Screen {
        components: vec![
            Box::new(Button {
                width: 10,
                height: 5,
                label: String::from("Hello"),
            }),
            Box::new(SelectBox {
                width: 75,
                height: 10,
                options: vec![
                    String::from("Yes"),
                    String::from("Maybe"),
                    String::from("No"),
                ],
            }),
        ],
    };
    screen.run();
}
