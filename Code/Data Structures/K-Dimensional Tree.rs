#[derive(Debug)]
struct Point {
    x: f64,
    y: f64,
}

#[derive(Debug)]
enum KdTree {
    Leaf(Point),
    Node {
        point: Point,
        left: Option<Box<KdTree>>,
        right: Option<Box<KdTree>>,
        dimension: usize,
    },
}

impl KdTree {
    fn new(points: Vec<Point>, depth: usize) -> Option<Box<KdTree>> {
        if points.is_empty() {
            return None;
        }

        let k = 2; // Number of dimensions

        let dimension = depth % k;
        let median_index = points.len() / 2;

        points.sort_by(|a, b| a.x.partial_cmp(&b.x).unwrap());

        Some(Box::new(KdTree::Node {
            point: points[median_index].clone(),
            left: KdTree::new(points[..median_index].to_vec(), depth + 1),
            right: KdTree::new(points[median_index + 1..].to_vec(), depth + 1),
            dimension,
        }))
    }

    fn nearest_neighbor(&self, query_point: &Point) -> Option<&Point> {
        self.nearest_neighbor_recursive(query_point, &self, 0)
    }

    fn nearest_neighbor_recursive(
        &self,
        query_point: &Point,
        current: &KdTree,
        depth: usize,
    ) -> Option<&Point> {
        match current {
            KdTree::Leaf(point) => Some(point),
            KdTree::Node {
                point,
                left,
                right,
                dimension,
            } => {
                let next_dimension = (dimension + 1) % 2; // For 2-dimensional space

                let (nearest_subtree, farthest_subtree) =
                    if query_point.coords()[*dimension] < point.coords()[*dimension] {
                        (left, right)
                    } else {
                        (right, left)
                    };

                let mut best = self.min_distance(&current, query_point, point);

                if let Some(nearest) = nearest_subtree {
                    if let Some(nearest_point) = self.nearest_neighbor_recursive(
                        query_point,
                        nearest,
                        depth + 1,
                    ) {
                        best = self.min_distance(&current, query_point, nearest_point);
                    }
                }

                if let Some(farthest) = farthest_subtree {
                    if let Some(farthest_point) = self.nearest_neighbor_recursive(
                        query_point,
                        farthest,
                        depth + 1,
                    ) {
                        best = self.min_distance(&current, query_point, farthest_point);
                    }
                }

                Some(best)
            }
        }
    }

    fn min_distance(&self, node: &KdTree, target: &Point, current_best: &Point) -> Point {
        let distance_node_target = self.distance(&node, target);

        if distance_node_target < current_best.distance(target) {
            node.point.clone()
        } else {
            current_best.clone()
        }
    }

    fn distance(&self, node: &KdTree, target: &Point) -> f64 {
        let dx = node.point.x - target.x;
        let dy = node.point.y - target.y;

        dx * dx + dy * dy
    }
}

trait Coords {
    fn coords(&self) -> Vec<f64>;
}

impl Coords for Point {
    fn coords(&self) -> Vec<f64> {
        vec![self.x, self.y]
    }
}

fn main() {
    let points = vec![
        Point { x: 2.0, y: 3.0 },
        Point { x: 5.0, y: 4.0 },
        Point { x: 9.0, y: 6.0 },
        Point { x: 4.0, y: 7.0 },
        Point { x: 8.0, y: 1.0 },
        Point { x: 7.0, y: 2.0 },
    ];

    let kdtree = KdTree::new(points.clone(), 0).unwrap();

    let query_point = Point { x: 9.0, y: 2.0 };
    let nearest_neighbor = kdtree.nearest_neighbor(&query_point);

    println!("Points: {:?}", points);
    println!("Query Point: {:?}", query_point);
    println!("Nearest Neighbor: {:?}", nearest_neighbor);
}
