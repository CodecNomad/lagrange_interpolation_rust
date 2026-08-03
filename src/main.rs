#[derive(Clone)]
struct Vec2 {
    x: f32,
    y: f32,
}

impl Vec2 {
    fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }
}

fn interpolate(x: f32, weights: Vec<Vec2>) -> f32 {
    let k = weights.len();

    let mut value = 0f32;
    for j in 0..k {
        let mut l = 1.0;

        fn calculate(x: f32, m: usize, j: usize, weights: &[Vec2]) -> f32 {
            (x - weights[m].x) / (weights[j].x - weights[m].x)
        }

        for m in 0..k {
            if m == j {
                continue;
            }

            let value = calculate(x, m, j, &weights);

            l *= value
        }

        value += weights[j].y * l;
    }

    value
}

fn main() {
    let value = interpolate(
        5f32,
        vec![
            Vec2::new(0f32, 2f32),
            Vec2::new(1f32, 3f32),
            Vec2::new(2f32, 4f32),
        ],
    );

    println!("{}", value)
}
