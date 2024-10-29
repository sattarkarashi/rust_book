use gui::{Draw, Button, Screen};

struct SelectBox {
    width: u32,
    height: u32,
    options: Vec<String>,
}

impl Draw for SelectBox {
    fn draw(&self){
        // Code to draw selectBox
    }
}

fn main {
    let screen = Screen {
        components: vec![Box::new(SelectBox {
            width:75,
            height: 10,
            option: vec![String::from("Yes"), String::from("Or"), String::from("Noe")],
        }),
        Box::new(Button {
            width:50,
            height: 10,
            label: String::from("Ok"),
        }),
        ],
    };

    screen.run();
}