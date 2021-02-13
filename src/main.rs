use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;

struct Box {
    x: i32, //left
    y: i32, //top
    size_x: i32,
    size_y: i32
}

impl Box {

    fn new(size_x: i32, size_y: i32) -> Box {
        Box{x: 0, y: 0, size_x, size_y}
    }

    fn draw(&self, color: &str, width: i32) -> Path{
        let data = Data::new()
            .move_to((self.x, self.y))
            .line_by((0, self.size_y))
            .line_by((self.size_x, 0))
            .line_by((0, -self.size_y))
            .close();

        let path = Path::new()
            .set("fill", "none")
            .set("stroke", color)
            .set("stroke-width", width)
            .set("d", data);

        return path;
    }

    fn draw_thin(&self, color: &str) -> Path{
        self.draw(color, 1)
    }

    fn draw_thick(&self, color: &str) -> Path{
        self.draw(color, 3)
    }

    fn translate(&self, x: i32, y: i32) -> Box{
        Box{x: x + self.x, y: y+ self.y, ..(*self)}
    }
}

/*fn main() {

    let boxes = [
        Box{size_x: 2, size_y: 2},
        Box{size_x: 1, size_y: 2},
        Box{size_x: 2, size_y: 1},
        Box{size_x: 1, size_y: 1}
    ];

    println!("Hello, world!");
}*/

fn main() {

    let colors = ["black", "blue", "green", "red", "yellow"];

    let border = Box{x: 10, y: 10, size_x: 30, size_y: 30};
    let boxes = [
        Box::new(20, 20),
        Box::new(10, 30),
        Box::new(30, 10),
        Box::new(10, 10)
    ];

    let mut document = Document::new()
        .set("viewBox", (0, 0, 70, 70))
        .add(border.draw_thick("black"));

    for (a, c) in boxes.iter().zip(colors[1..].iter().cycle()) {
        document = document
            .add(a.translate(10,10).draw_thin(c));
    }

    svg::save("image.svg", &document).unwrap();

}
