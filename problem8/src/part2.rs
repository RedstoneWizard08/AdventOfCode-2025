use std::{
    collections::{HashMap, HashSet},
    hash::Hash,
};

use common::Squared;
use itertools::Itertools;
use macros::embed_lines;

embed_lines!("../input.txt");

pub type NetworkId = usize;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Default)]
pub struct Pos {
    pub x: i32,
    pub y: i32,
    pub z: i32,
}

impl Pos {
    pub const fn new() -> Self {
        Self { x: 0, y: 0, z: 0 }
    }

    /// Get the squared distance to the [`other`] point.
    pub fn dist_sqr(&self, other: Pos) -> f32 {
        let x1 = self.x as f32;
        let x2 = other.x as f32;

        let y1 = self.y as f32;
        let y2 = other.y as f32;

        let z1 = self.z as f32;
        let z2 = other.z as f32;

        let x = (x2 - x1).sqr();
        let y = (y2 - y1).sqr();
        let z = (z2 - z1).sqr();

        x + y + z
    }

    /// Get the distance to the [`other`] point.
    ///
    /// >> NOTE:
    /// If possible, use [`Self::dist`] instead for performance,
    /// since although square roots are quite fast, it's less instructions
    /// to execute without it.
    pub fn dist(&self, other: Pos) -> f32 {
        self.dist_sqr(other).sqrt()
    }
}

#[derive(Debug, Clone, Default)]
pub struct Network {
    pub id: NetworkId,
    pub boxes: HashSet<Pos>,
}

#[derive(Debug, Clone, Default)]
pub struct ConnectionSet {
    connections: HashMap<Pos, HashSet<Pos>>,
    network_refs: HashMap<Pos, NetworkId>,
    networks: HashMap<usize, Network>,

    /// Used to generate a network ID.
    network_count: usize,
}

impl ConnectionSet {
    pub fn connect(&mut self, a: Pos, b: Pos) -> bool {
        self.connections.entry(a).or_default().insert(b);
        self.connections.entry(b).or_default().insert(a);

        let net_a = self.network_refs.get(&a).copied();
        let net_b = self.network_refs.get(&b).copied();

        match (net_a, net_b) {
            (Some(net_id), None) | (None, Some(net_id)) => {
                let net = self.networks.get_mut(&net_id).unwrap();

                net.boxes.insert(a);
                net.boxes.insert(b);

                self.network_refs.insert(a, net_id);
                self.network_refs.insert(b, net_id);

                false
            }

            (Some(net_a_id), Some(net_b_id)) => {
                self.network_count += 1;

                let mut net = Network {
                    id: self.network_count,
                    boxes: HashSet::new(),
                };

                let net_a = self.networks.remove(&net_a_id).unwrap();
                let net_b = self.networks.remove(&net_b_id).unwrap_or_default(); // unwrap_or_default() in case net_a_id == net_b_id

                net.boxes.extend(net_a.boxes);
                net.boxes.extend(net_b.boxes);
                net.boxes.insert(a);
                net.boxes.insert(b);

                for pos in &net.boxes {
                    self.network_refs.insert(*pos, net.id);
                }

                self.network_refs.insert(a, net.id);
                self.network_refs.insert(b, net.id);

                self.networks.insert(net.id, net);

                true
            }

            (None, None) => {
                self.network_count += 1;

                let mut net = Network {
                    id: self.network_count,
                    boxes: HashSet::new(),
                };

                net.boxes.insert(a);
                net.boxes.insert(b);

                self.network_refs.insert(a, net.id);
                self.network_refs.insert(b, net.id);

                self.networks.insert(net.id, net);

                false
            }
        }
    }

    pub fn get_network(&self, pos: Pos) -> Option<&Network> {
        self.network_refs
            .get(&pos)
            .map(|it| self.networks.get(it))
            .flatten()
    }

    pub fn has(&self, a: Pos, b: Pos) -> bool {
        self.connections.get(&a).is_some_and(|it| it.contains(&b))
            || self.connections.get(&b).is_some_and(|it| it.contains(&a))
    }

    pub fn has_any(&self, pos: Pos) -> bool {
        self.connections.get(&pos).is_some_and(|it| !it.is_empty())
    }

    pub fn has_all(&self, set: [Pos; HEIGHT]) -> bool {
        set.iter().all(|it| self.has_any(*it))
    }
}

#[derive(Debug, Clone)]
pub struct World {
    pub boxes: [Pos; HEIGHT],
    pub connections: ConnectionSet,
}

impl World {
    pub fn new() -> Self {
        Self {
            boxes: [Pos::new(); HEIGHT],
            connections: ConnectionSet::default(),
        }
    }
}

pub fn main() {
    let mut world = World::new();

    for (i, line) in INPUT.iter().enumerate() {
        let (x, y, z) = line.splitn(3, ",").collect_tuple::<(_, _, _)>().unwrap();

        world.boxes[i].x = x.parse::<i32>().unwrap();
        world.boxes[i].y = y.parse::<i32>().unwrap();
        world.boxes[i].z = z.parse::<i32>().unwrap();
    }

    let permutations = world
        .boxes
        .iter()
        .copied()
        .permutations(2)
        .map(|it| it.into_iter().collect_tuple::<(_, _)>().unwrap())
        .filter(|(a, b)| a != b)
        .map(|(a, b)| (a, b, a.dist_sqr(b).abs() as i32))
        .sorted_unstable_by_key(|(_, _, d)| *d)
        .collect_vec();

    let mut last = None;

    while !world.connections.has_all(world.boxes) || world.connections.networks.len() != 1 {
        let closest = permutations
            .iter()
            .filter(|(pos, it, _)| !world.connections.has(*pos, *it))
            .next();

        if let Some((pos, closest, _)) = closest {
            world.connections.connect(*pos, *closest);
            last = Some((*pos, *closest));
        }
    }

    let res = last.unwrap();
    let res = res.0.x as i64 * res.1.x as i64;

    std::hint::black_box(res);

    #[cfg(feature = "cli")]
    println!("Result: {res}");
}
