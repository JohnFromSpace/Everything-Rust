use std::collections::HashMap;

#[derive(Debug, Clone, PartialEq, Eq, Hash)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
struct CartesianGraph {
    points: HashMap<String, Point>,
}

impl CartesianGraph {
    fn new() -> Self {
        CartesianGraph {
            points: HashMap::new(),
        }
    }

    fn add_point(&mut self, name: &str, x: f64, y: f64) {
        let point = Point { x, y };
        self.points.insert(name.to_string(), point);
    }

    fn get_point(&self, name: &str) -> Option<&Point> {
        self.points.get(name)
    }

    fn plot_points(&self) {
        for (name, point) in &self.points {
            println!("Point {}: ({}, {})", name, point.x, point.y);
        }
    }
}

fn main() {
    let mut cartesian_graph = CartesianGraph::new();

    // Add points
    cartesian_graph.add_point("A", 1.0, 2.0);
    cartesian_graph.add_point("B", 3.0, 4.0);
    cartesian_graph.add_point("C", -1.0, 0.0);

    // Retrieve and print points
    if let Some(point) = cartesian_graph.get_point("B") {
        println!("Point B: ({}, {})", point.x, point.y);
    } else {
        println!("Point not found.");
    }

    // Plot all points
    cartesian_graph.plot_points();
}
