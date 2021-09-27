use cpython::{PyDict, Python, ToPyObject};
use rosu_pp::{Beatmap, BeatmapExt, ManiaPP};
use std::fs::File;

pub(crate) struct ManiaResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
}

impl ToPyObject for ManiaResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars)
            .unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();

        dict
    }
}

pub(crate) fn calculate_mania_pp(
    map: String,
    mods: u32,
    score: Option<u32>,
    passed_objects: Option<usize>,
) -> ManiaResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = ManiaPP::new(&map).mods(mods);

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result,
    };

    let result = match score {
        Some(x) => result.score(x),
        None => result,
    };

    let stats = ManiaResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.calculate().pp(),
    };

    return stats;
}
