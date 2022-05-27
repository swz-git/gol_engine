use std::num;

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
pub struct Point {
    pub x: i32,
    pub y: i32,
}

pub fn serialize(input: Vec<Point>) -> String {
    let mut output = String::new();
    for point in input {
        output.push_str(&format!("{},{}\n", point.x, point.y));
    }
    return output;
}

fn get_bounding_area(input: &Vec<Point>) -> (Point, Point) {
    let mut min_x = std::i32::MAX;
    let mut max_x = std::i32::MIN;
    let mut min_y = std::i32::MAX;
    let mut max_y = std::i32::MIN;
    for point in input {
        if point.x < min_x {
            min_x = point.x;
        }
        if point.x > max_x {
            max_x = point.x;
        }
        if point.y < min_y {
            min_y = point.y;
        }
        if point.y > max_y {
            max_y = point.y;
        }
    }
    return (Point { x: min_x, y: min_y }, Point { x: max_x, y: max_y });
}

fn pretty_serialize(input: &Vec<Point>) -> String {
    let (min, max) = get_bounding_area(input);
    let mut output = String::new();
    for y in min.y..=max.y {
        for x in min.x..=max.x {
            if input.contains(&Point { x: x, y: y }) {
                output.push_str("#");
            } else {
                output.push_str(".");
            }
        }
        output.push_str("\n");
    }
    return output;
}

pub fn deserialize(input: String) -> Vec<Point> {
    let mut cells: Vec<Point> = Vec::new();
    for (_line, strval) in input.lines().enumerate() {
        let pointsplit: Vec<&str> = strval.split(",").collect();
        let x = pointsplit[0]
            .parse::<i32>()
            .expect("Failed to parse x, invalid input");
        let y = pointsplit[1]
            .parse::<i32>()
            .expect("Failed to parse y, invalid input");
        cells.push(Point { x, y });
    }
    return cells;
}

pub fn get_cells_to_compute(living_cells: &Vec<Point>) -> Vec<Point> {
    let mut cells_to_compute: Vec<Point> = living_cells.to_vec();
    for cell in living_cells {
        cells_to_compute.push(Point {
            x: cell.x - 1,
            y: cell.y - 1,
        });
        cells_to_compute.push(Point {
            x: cell.x - 1,
            y: cell.y,
        });
        cells_to_compute.push(Point {
            x: cell.x - 1,
            y: cell.y + 1,
        });
        cells_to_compute.push(Point {
            x: cell.x,
            y: cell.y - 1,
        });
        cells_to_compute.push(Point {
            x: cell.x,
            y: cell.y + 1,
        });
        cells_to_compute.push(Point {
            x: cell.x + 1,
            y: cell.y - 1,
        });
        cells_to_compute.push(Point {
            x: cell.x + 1,
            y: cell.y,
        });
        cells_to_compute.push(Point {
            x: cell.x + 1,
            y: cell.y + 1,
        });
    }
    cells_to_compute.sort();
    cells_to_compute.dedup();
    return cells_to_compute;
}

fn get_cell_neighbors(cell: Point) -> Vec<Point> {
    let mut neighbors: Vec<Point> = Vec::new();
    neighbors.push(Point {
        x: cell.x - 1,
        y: cell.y - 1,
    });
    neighbors.push(Point {
        x: cell.x - 1,
        y: cell.y,
    });
    neighbors.push(Point {
        x: cell.x - 1,
        y: cell.y + 1,
    });
    neighbors.push(Point {
        x: cell.x,
        y: cell.y - 1,
    });
    neighbors.push(Point {
        x: cell.x,
        y: cell.y + 1,
    });
    neighbors.push(Point {
        x: cell.x + 1,
        y: cell.y - 1,
    });
    neighbors.push(Point {
        x: cell.x + 1,
        y: cell.y,
    });
    neighbors.push(Point {
        x: cell.x + 1,
        y: cell.y + 1,
    });
    return neighbors;
}

pub fn get_living_cell_neighbors(cell: Point, living_cells: &Vec<Point>) -> Vec<Point> {
    let mut living_cell_neighbors = living_cells.clone();
    living_cell_neighbors
        .retain(|other| (cell.x - other.x).abs() <= 1 && (cell.y - other.y).abs() <= 1);
    return living_cell_neighbors;
}

pub fn tick(living_cells: &mut Vec<Point>) {
    let cells_to_compute = get_cells_to_compute(living_cells);
    let mut new_living_cells: Vec<Point> = living_cells.clone();
    for cell in cells_to_compute {
        let living_cell_neighbor_count =
            get_living_cell_neighbors(cell.clone(), &living_cells).len();

        if (living_cells.contains(&cell)) {
            // cell is alive
            if living_cell_neighbor_count < 2 {
                // underpopulation
                new_living_cells.retain(|x| x != &cell);
            } else if living_cell_neighbor_count > 3 {
                // overpopulation
                new_living_cells.retain(|x| x != &cell);
            }
        } else {
            // cell is dead
            if living_cell_neighbor_count == 3 {
                // reproduction
                new_living_cells.push(cell);
            }
        }
    }
    *living_cells = new_living_cells;
}
