use super::parse::parse_map;
use cpython::{PyDict, Python, ToPyObject};
use rosu_pp::{BeatmapExt, OsuPP};

pub(crate) struct StdResults {
    total_stars: f64,
    partial_stars: f64,
    pp: f64,
    max_pp: f64,
    max_combo: usize,
    ar: f64,
    cs: f64,
    od: f64,
    hp: f64,
    clock_rate: f64,
}

impl ToPyObject for StdResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars)
            .unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();
        dict.set_item(py, "max_pp", self.max_pp).unwrap();
        dict.set_item(py, "max_combo", self.max_combo).unwrap();
        dict.set_item(py, "ar", self.ar).unwrap();
        dict.set_item(py, "cs", self.cs).unwrap();
        dict.set_item(py, "od", self.od).unwrap();
        dict.set_item(py, "hp", self.hp).unwrap();
        dict.set_item(py, "clock_rate", self.clock_rate).unwrap();

        dict
    }
}

pub(crate) fn calculate_std_pp(
    file_path: String,
    mods: u32,
    combo: Option<usize>,
    acc: Option<f64>,
    potential_acc: Option<f64>,
    n300: Option<usize>,
    n100: Option<usize>,
    n50: Option<usize>,
    nmiss: Option<usize>,
    passed_objects: Option<usize>,
) -> StdResults {
    let map = parse_map(file_path);

    let result = OsuPP::new(&map).mods(mods);

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

    let result = match n50 {
        Some(x) => result.n50(x),
        None => result,
    };

    let result = match acc {
        Some(x) => result.accuracy(x),
        None => result,
    };

    let potential_result = OsuPP::new(&map).mods(mods).misses(0);

    let map_attributes = map.attributes().mods(mods);

    let potential_result = match potential_acc {
        Some(x) => potential_result.accuracy(x),
        None => potential_result,
    };

    let result = result.calculate();

    StdResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: result.stars(),
        pp: result.pp,
        max_pp: potential_result.calculate().pp,
        max_combo: result.attributes.max_combo,
        ar: map_attributes.ar,
        cs: map_attributes.cs,
        od: map_attributes.od,
        hp: map_attributes.hp,
        clock_rate: map_attributes.clock_rate,
    }
}
