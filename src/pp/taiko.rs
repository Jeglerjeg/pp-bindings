use cpython::{PyDict, Python, ToPyObject};
use rosu_pp::{Beatmap, BeatmapExt, TaikoPP};
use std::fs::File;

pub(crate) struct TaikoResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
}

impl ToPyObject for TaikoResults {
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

pub(crate) fn calculate_taiko_pp(
    map: String,
    mods: u32,
    combo: Option<usize>,
    acc: Option<f32>,
    n300: Option<usize>,
    n100: Option<usize>,
    nmiss: Option<usize>,
    passed_objects: Option<usize>,
) -> TaikoResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = TaikoPP::new(&map).mods(mods);

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result,
    };

    let result = match combo {
        Some(x) => result.combo(x),
        None => result,
    };

    let result = match nmiss {
        Some(x) => result.misses(x),
        None => result.misses(0),
    };

    let result = match n300 {
        Some(x) => result.n300(x),
        None => result,
    };

    let result = match n100 {
        Some(x) => result.n100(x),
        None => result,
    };

    let result = match acc {
        Some(x) => result.accuracy(x),
        None => result,
    };

    let result = result.calculate();

    TaikoResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: result.stars(),
        pp: result.pp(),
    }
}
