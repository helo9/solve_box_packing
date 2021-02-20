use std::cmp;
use svg::Document;
use svg::node::element::Path;
use svg::node::element::path::Data;
use itertools::izip;
use itertools::iproduct;

#[derive(Debug)]
struct Position {
    x: i32,
    y: i32
}

impl Position {
    fn new(x: i32, y:i32) -> Position{
        Position{x, y}
    }
}

#[derive(Debug)]
#[derive(Clone)]
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

    fn draw(&self, color: &str, width: i32) -> Path {
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

    fn draw_thin(&self, color: &str) -> Path {
        self.draw(color, 1)
    }

    fn draw_thick(&self, color: &str) -> Path {
        self.draw(color, 2)
    }

    fn translate(&self, x: i32, y: i32) -> Box {
        Box{x: x + self.x, y: y+ self.y, ..(*self)}
    }

    fn translate2(&self, translation: &Position) -> Box {
        Box{x: translation.x + self.x, y: translation.y + self.y, ..*self}
    }

    fn get_position(&self) -> Position {
        Position{x: self.x, y: self.y}
    }

    fn contains(&self, other: &Box) -> bool {
        /*
         -- > x
         |
         |
         v y
        */
        let northwest_inside = self.x <= other.x && self.y <= other.y;
        let southeast_inside = ( self.x + self.size_x ) >= ( other.x + other.size_x )
            && ( self.y + self.size_y ) >= (other.y + other.size_y);

        return northwest_inside && southeast_inside;
    }

    fn intersects(&self, other: &Box) -> bool {
        println!("{:?} and {:?}", self, other);
        println!("self.intersects(other)..");
        let a = self._intersects(other);
        println!("self.intersects(other) = {}", a);
        println!("other.intersects(self)..");
        let b = other._intersects(self);
        println!("other.intersects(self) = {}", b);

        a || b
    }

    fn _intersects(&self, other: &Box) -> bool {
        let nw = Position::new(self.x, self.y);
        let ne = Position::new(self.x+self.size_x, self.y);
        let se = Position::new(self.x+self.size_x, self.y+self.size_y);
        let sw = Position::new(self.x, self.y+self.size_y);

        let a = (nw.x >= other.x) 
          && (nw.x < other.x + other.size_x)
          && (nw.y >= other.y)
          && (nw.y < other.y + other.size_y);

        let b = (ne.x > other.x)
          && (ne.x <= other.x + other.size_x)
          && (ne.y >= other.y)
          && (ne.y < other.y + other.size_y);

        let c = (se.x > other.x)
          && (se.x <= other.x+other.size_x)
          && (se.y > other.y)
          && (se.y <= other.y+other.size_y);

        let d = (sw.x >= other.x)
          && (sw.x < other.x+other.size_x)
          && (sw.y > other.y)
          && (sw.y <= other.y+other.size_y);
        
        println!("{:?}, {:?}, {:?}, {:?}", nw, ne, se, sw);

        println!("{} || {} || {} || {}", a, b, c, d);
       
        a || b || c || d 

    }
}


struct Drawer {
    objects: Vec<Path>,
    x_max: i32,
    y_max: i32
}

impl Drawer {
    const COLORS: [&'static str; 5] = ["black", "blue", "green", "red", "yellow"];

    fn new() -> Drawer {
        Drawer{objects: Vec::new(), x_max: 0, y_max: 0}
    }

    fn add_problem(&mut self, border: &Box, boxes: &[Box]){

        let mut x_pos = 10;
    
        self.objects.push(border.translate(x_pos,10).draw_thick("black"));
    
        x_pos += border.size_x + 10;
    
        for (a, c) in boxes.iter().zip(Drawer::COLORS[1..].iter().cycle()) {
            self.objects.push(a.translate(x_pos,10+self.y_max).draw_thin(c));
            x_pos += a.size_x + 10;
        }

        self.x_max = cmp::max(self.x_max, x_pos);

        self.y_max += border.size_y + 20;
    }

    fn add_positions(&mut self, solutions: &Vec<Vec<Position>>, border: &Box, boxes: &[Box]) {
        for (solution, abox, color) in izip!(solutions.iter(), boxes.iter(), Drawer::COLORS[1..].iter()) {
            self.add_position(solution, border, abox, color);
        }
    }

    fn add_position(&mut self, solution: &Vec<Position>, border: &Box, abox: &Box, color: &str) {

        let mut x_pos = 10;

        for solution_part in solution.iter() {
            self.objects.push(
                border.translate(x_pos, self.y_max + 10).draw_thick("black")
            );

            self.objects.push(
                abox.translate(x_pos+solution_part.x, 10 + self.y_max+solution_part.y).draw_thin(color)
            );
            
            x_pos += border.size_x + 20;
        }
    
        self.y_max += border.size_x + 20;

        self.x_max = cmp::max(x_pos, self.x_max);
    }

    fn _add_solution(&mut self, border: &Box, solution: &Vec<Box>, x_pos: i32, y_pos: i32) {

        self.objects.push(
            border.translate(x_pos, y_pos).draw_thick("black")
        );

        for (abox, color) in izip!(solution.iter(), Drawer::COLORS[1..].iter()) {
            self.objects.push(
                abox.translate(x_pos, y_pos).draw_thin(color)
            );
        }

    }

    fn add_solutions(&mut self, border: &Box, solutions: &Vec<Vec<Box>>) {
        let mut x_pos = 10;
        self.y_max += 10;

        for solution in solutions {
            if x_pos + border.size_x >= self.x_max {
                x_pos = 10;
                self.y_max += border.size_y + 10;
            }

            self._add_solution(border, solution, x_pos, self.y_max);

            x_pos += border.size_x + 20;
        }

        self.y_max += border.size_x + 10;
    }

    fn draw(&mut self) {

        let mut document = Document::new();

        for object in self.objects.iter() {
            document = document.add(object.clone());
        }

        document = document.set("viewBox", (0, 0, self.x_max, self.y_max));

        svg::save("image.svg", &document).unwrap();
    }
}

fn main() {

    let mut drawer = Drawer::new();

    // Problem Definition
    let border = Box::new(30, 30);
    let boxes = [
        Box::new(20, 20),
        Box::new(10, 20),
        Box::new(30, 5),
        Box::new(30, 5)/*
        Box::new(30, 5),
        Box::new(30, 10),
        Box::new(30, 15)*/
    ];

    drawer.add_problem(&border, &boxes);

    // span up solution space
    let  x_sizes: Vec<i32> = boxes.iter()
        .map(|abox| abox.size_x).collect();
    let  y_sizes: Vec<i32> = boxes.iter()
        .map(|abox| abox.size_y).collect();
    
    print!("x sizes: {:?}\n", &x_sizes);
    print!("y sizes: {:?}\n", &y_sizes);

    let mut solution_space: Vec<Vec<Position>> = Vec::new();

    fn collect_positions(current_sum: i32, summands: &[i32], maximum: i32) -> Vec<i32> {
        if current_sum < maximum {
            let mut sums: Vec<i32> = Vec::new();

            sums.push(current_sum);

            for summand in summands.iter() {
                sums.extend(collect_positions(current_sum + summand, summands, maximum));
            }
            
            sums.sort();
            sums.dedup();
            return sums;            
        } else {
            return Vec::<i32>::new();
        }
    }

    let x_positions = collect_positions(0, &x_sizes, border.size_x);
    let y_positions = collect_positions(0, &y_sizes, border.size_y);

    println!("x_positions: {:?}", x_positions);
    println!("y_positions: {:?}", y_positions);

    for abox in boxes.iter() {
        let mut box_positions: Vec<Position> = Vec::new();

        for (x, y) in iproduct!(x_positions.iter(), y_positions.iter()){
            let newbox = abox.translate(*x, *y);
            if border.contains(&newbox) {
                box_positions.push(newbox.get_position());
            } else {
                continue;
            }
        }

        solution_space.push(box_positions);
    }

    drawer.add_positions(&solution_space, &border, &boxes);

    // Collect solutions

    fn collect_solutions(boxes: &[Box], solution_space: &[Vec<Position>], solution: Vec<Box>) -> Vec<Vec<Box>> {

        let mut solutions: Vec<Vec<Box>> = Vec::new();
                
        if boxes.len() == 0 {
            solutions.push(solution);
            return solutions;
        }


        for box_position in solution_space[0].iter() {
            let newbox = boxes[0].translate2(box_position);
            let mut collision: bool = false;
            for abox in solution.iter() {
                collision |= abox.intersects(&newbox);
                if collision { break; }
            }
            
            if !collision {
                let mut tmp_solution = solution.clone();
                tmp_solution.push(newbox);

                let tmp_solutions = collect_solutions(&boxes[1..], &solution_space[1..], tmp_solution);

                solutions.extend(tmp_solutions);
            }   
        }
        return solutions;
        
    }

    let sol = collect_solutions(&boxes, &solution_space, Vec::<Box>::new());

    println!("solutions:\n{:#?}", sol);

    if sol.len() > 0 {
        drawer.add_solutions(&border, &sol);
    }

    drawer.draw();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn intersection_works() {
        let a = Box{x: 0, y: 0, size_x: 20, size_y: 20};
        let b = Box{x: 10, y: 10, size_x: 20, size_y: 20};
        let b0 = Box{x:20, y:0, size_x: 20, size_y: 20};
        let c = Box{x: 0, y: 5, size_x: 30, size_y: 5};

        let d = Box{x: 0, y: 25, size_x: 30, size_y: 5};
        let e = Box{x: 0, y: 15, size_x: 30, size_y: 10};

        assert!(a.intersects(&b));
        assert!(!a.intersects(&b0));
        assert!(a.intersects(&c));
        assert!(!d.intersects(&e));
    }
}