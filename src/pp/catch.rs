use cpython::{PyDict, Python, ToPyObject};
use rosu_pp::fruits::FruitsAttributeProvider;
use rosu_pp::{Beatmap, BeatmapExt, FruitsPP};
use std::fs::File;

pub(crate) struct CatchResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
    max_combo: usize,
}

impl ToPyObject for CatchResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars)
            .unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();
        dict.set_item(py, "max_combo", self.max_combo).unwrap();

        dict
    }
}

pub(crate) fn calculate_catch_pp(
    map: String,
    mods: u32,
    combo: Option<usize>,
    fruits: Option<usize>,
    droplets: Option<usize>,
    tiny_droplets: Option<usize>,
    tiny_droplet_misses: Option<usize>,
    nmiss: Option<usize>,
    passed_objects: Option<usize>,
) -> CatchResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = FruitsPP::new(&map).mods(mods);

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result,
    };

    let result = match nmiss {
        Some(x) => result.misses(x),
        None => result.misses(0),
    };

    let result = match combo {
        Some(x) => result.combo(x),
        None => result,
    };

    let result = match fruits {
        Some(x) => result.fruits(x),
        None => result,
    };

    let result = match droplets {
        Some(x) => result.droplets(x),
        None => result,
    };

    let result = match tiny_droplets {
        Some(x) => result.tiny_droplets(x),
        None => result,
    };

    let result = match tiny_droplet_misses {
        Some(x) => result.tiny_droplet_misses(x),
        None => result,
    };

    let result = result.calculate();

    let stats = CatchResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.pp(),
        max_combo: result.attributes().unwrap().max_combo,
    };

    return stats;
}
