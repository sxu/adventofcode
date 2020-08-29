use itertools::Itertools;
use std::collections::hash_map::Entry;
use std::collections::HashMap;
use std::fs::File;
use std::io::{self, BufRead};

pub fn day6(input_path: &str) {
    let file = File::open(input_path).unwrap();
    let mut orbits = PlanetarySystem::new();
    for line in io::BufReader::new(file).lines() {
        let line = line.unwrap();
        let (planet, satellite) = line.trim().split(")").next_tuple().unwrap();
        orbits.add_orbit(planet, satellite);
    }
    let com_id = orbits.get_node_id("COM");
    assert_eq!(orbits.total_orbits(com_id), 171213);

    let you_id = orbits.get_node_id("YOU");
    let santa_id = orbits.get_node_id("SAN");
    assert_eq!(orbits.num_transfers(com_id, you_id, santa_id), 292);
}

struct Object {
    satellite_ids: Vec<usize>,
}

impl Object {
    fn new() -> Object {
        Object {
            satellite_ids: Vec::new(),
        }
    }
}

struct PlanetarySystem {
    objects: Vec<Object>,
    node_id_map: HashMap<String, usize>,
}

impl PlanetarySystem {
    fn new() -> PlanetarySystem {
        PlanetarySystem {
            objects: Vec::new(),
            node_id_map: HashMap::new(),
        }
    }

    fn add_orbit(&mut self, planet_name: &str, satellite_name: &str) {
        let planet_id = self.get_node_id(planet_name);
        let satellite_id = self.get_node_id(satellite_name);
        self.objects[planet_id].satellite_ids.push(satellite_id);
    }

    fn get_node_id(&mut self, name: &str) -> usize {
        match self.node_id_map.entry(String::from(name)) {
            Entry::Occupied(o) => *o.get(),
            Entry::Vacant(v) => {
                let node_id = self.objects.len();
                self.objects.push(Object::new());
                v.insert(node_id);
                node_id
            }
        }
    }

    fn total_orbits(&self, node_id: usize) -> usize {
        let (total_orbits, _) = self.total_orbits_and_objects(node_id);
        total_orbits
    }

    fn total_orbits_and_objects(&self, node_id: usize) -> (usize, usize) {
        let mut total_orbits = 0;
        let mut total_objects = 1;
        for satellite_id in self.objects[node_id].satellite_ids.iter() {
            let (orbits, objects) = self.total_orbits_and_objects(*satellite_id);
            total_orbits += orbits + objects;
            total_objects += objects;
        }
        (total_orbits, total_objects)
    }

    fn num_transfers(&self, root_id: usize, from_id: usize, to_id: usize) -> usize {
        match self.num_transfers_helper(root_id, from_id, to_id).unwrap() {
            TransferResult::Both(x) => x,
            _ => panic!("Unreachable"),
        }
    }

    fn num_transfers_helper(
        &self,
        current_id: usize,
        from_id: usize,
        to_id: usize,
    ) -> Option<TransferResult> {
        if current_id == from_id {
            return Some(TransferResult::One(0));
        }
        if current_id == to_id {
            return Some(TransferResult::One(0));
        }
        let rec_results = self.objects[current_id]
            .satellite_ids
            .iter()
            .filter_map(|s| self.num_transfers_helper(*s, from_id, to_id))
            .collect::<Vec<TransferResult>>();
        match rec_results[..] {
            [] => None,
            [TransferResult::Both(x)] => Some(TransferResult::Both(x)),
            [TransferResult::One(x)] => Some(TransferResult::One(x + 1)),
            [TransferResult::One(x), TransferResult::One(y)] => Some(TransferResult::Both(x + y)),
            _ => panic!("Unreachable"),
        }
    }
}

enum TransferResult {
    One(usize),
    Both(usize),
}
