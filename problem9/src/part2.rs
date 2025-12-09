use indicatif::ParallelProgressIterator;
use itertools::Itertools;
use macros::embed_positions;
use parry2d::{math::Point, query::PointQuery, shape::ConvexPolygon};
use rayon::iter::{IntoParallelIterator, ParallelIterator};

embed_positions!("../input.txt");

pub fn main() {
    let input = INPUT
        .into_iter()
        .map(|it| Point::new(it.0 as f32, it.1 as f32))
        .collect_vec();

    // let shape = Voxels::from_points(Vector2::new(1.0, 1.0), &input);
    let shape = ConvexPolygon::from_convex_hull(&input).unwrap();

    let max = INPUT
        .into_iter()
        .permutations(2)
        .map(|it| it.into_iter().collect_tuple::<(_, _)>().unwrap())
        .filter(|((x1, y1), (x2, y2))| *x1 < *x2 && *y1 < *y2)
        .collect_vec()
        .into_par_iter()
        .progress()
        .filter(|((x1, y1), (x2, y2))| {
            !((*x1 + 1)..*x2)
                .into_iter()
                .flat_map(|x| {
                    ((*y1 + 1)..*y2)
                        .into_iter()
                        .map(move |y| Point::new(x as f32, y as f32))
                })
                .any(|it| shape.contains_local_point(&it))
        })
        .map(|((x1, y1), (x2, y2))| (x2 - x1 + 1) * (y2 - y1 + 1))
        .max()
        .unwrap();

    println!("Max: {max}");
}
