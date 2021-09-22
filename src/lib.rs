extern crate cpython;

use std::fs::File;

use cpython::{py_fn, py_module_initializer, PyResult, Python, ToPyObject, PyDict};
use rosu_pp::{Beatmap, BeatmapExt, OsuPP, TaikoPP, ManiaPP, FruitsPP};
mod osu;

py_module_initializer!(pp_bindings, |py, m| {
    m.add(py, "__doc__", "This module is implemented in Rust.")?;
    m.add(py, "std_pp", py_fn!(py, calculate_std_pp_py(map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, potential_acc: Option<f32>, n300: Option<usize>,
                n100: Option<usize>, n50: Option<usize>, nmiss: Option<usize>, passed_objects: Option<usize>)))?;
    m.add(py, "taiko_pp", py_fn!(py, calculate_taiko_pp_py(map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, n300: Option<usize>,
                         n100: Option<usize>, nmiss: Option<usize>, passed_objects: Option<usize>)))?;
    m.add(py, "mania_pp", py_fn!(py, calculate_mania_pp_py(map: String, mods: u32, score: Option<u32>, passed_objects: Option<usize>)))?;
    m.add(py, "catch_pp", py_fn!(py, calculate_catch_pp_py(map: String, mods: u32, combo: Option<usize>, fruits: Option<usize>,
                         droplets: Option<usize>, tiny_droplets: Option<usize>,
                         tiny_droplet_misses: Option<usize>, nmiss: Option<usize>,
                         passed_objects: Option<usize>)))?;
    Ok(())
});

struct StdResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
    max_pp: f32
}

struct TaikoResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
}

struct ManiaResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
}

struct CatchResults {
    total_stars: f32,
    partial_stars: f32,
    pp: f32,
}

impl ToPyObject for StdResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars).unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();
        dict.set_item(py, "max_pp", self.max_pp).unwrap();

        dict
    }
}

impl ToPyObject for TaikoResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars).unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();

        dict
    }
}

impl ToPyObject for ManiaResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars).unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();

        dict
    }
}

impl ToPyObject for CatchResults {
    type ObjectType = PyDict;

    fn to_py_object(&self, py: Python) -> PyDict {
        let dict = PyDict::new(py);
        dict.set_item(py, "total_stars", self.total_stars).unwrap();
        dict.set_item(py, "partial_stars", self.partial_stars).unwrap();
        dict.set_item(py, "pp", self.pp).unwrap();

        dict
    }
}

fn calculate_std_pp(map: String, mods: u32, combo: Option<usize>, acc: Option<f32>,
                    potential_acc: Option<f32>, n300: Option<usize>, n100: Option<usize>,
                    n50: Option<usize>, nmiss: Option<usize>,
                    passed_objects: Option<usize>) -> StdResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = OsuPP::new(&map)
        .mods(mods);

    let result = match combo {
        Some(x) => result.combo(x),
        None => result
    };

    let result = match acc {
        Some(x) => result.accuracy(x),
        None => result
    };

    let result = match n300 {
        Some(x) => result.n300(x),
        None => result
    };

    let result = match n100 {
        Some(x) => result.n100(x),
        None => result
    };

    let result = match n50 {
        Some(x) => result.n50(x),
        None => result
    };

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result
    };

    let result = match nmiss {
        Some(x) => result.misses(x),
        None => result.misses(0)
    };

    let new_result = OsuPP::new(&map)
        .mods(mods)
        .misses(0);

    let new_result = match potential_acc {
        Some(x) => new_result.accuracy(x),
        None => new_result
    };

    let stats = StdResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.calculate().pp(),
        max_pp: new_result.calculate().pp()
    };

    return stats;
}

fn calculate_taiko_pp(map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, n300: Option<usize>,
                      n100: Option<usize>, nmiss: Option<usize>,
                      passed_objects: Option<usize>) -> TaikoResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = TaikoPP::new(&map)
        .mods(mods);

    let result = match combo {
        Some(x) => result.combo(x),
        None => result
    };

    let result = match acc {
        Some(x) => result.accuracy(x),
        None => result
    };

    let result = match n300 {
        Some(x) => result.n300(x),
        None => result
    };

    let result = match n100 {
        Some(x) => result.n100(x),
        None => result
    };

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result
    };

    let result = match nmiss {
        Some(x) => result.misses(x),
        None => result.misses(0)
    };

    let stats = TaikoResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.calculate().pp(),
    };

    return stats;
}

fn calculate_mania_pp(map: String, mods: u32, score: Option<u32>,
                      passed_objects: Option<usize>) -> ManiaResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = ManiaPP::new(&map)
        .mods(mods);

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result
    };

    let result = match score {
        Some(x) => result.score(x),
        None => result
    };

    let stats = ManiaResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.calculate().pp(),
    };

    return stats;
}

fn calculate_catch_pp(map: String, mods: u32, combo: Option<usize>, fruits: Option<usize>,
                      droplets: Option<usize>, tiny_droplets: Option<usize>,
                      tiny_droplet_misses: Option<usize>, nmiss: Option<usize>,
                      passed_objects: Option<usize>) -> CatchResults {
    let file = match File::open(map) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    let map = match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };

    let result = FruitsPP::new(&map)
        .mods(mods);

    let result = match combo {
        Some(x) => result.combo(x),
        None => result
    };

    let result = match fruits {
        Some(x) => result.fruits(x),
        None => result
    };

    let result = match droplets {
        Some(x) => result.droplets(x),
        None => result
    };

    let result = match tiny_droplets {
        Some(x) => result.tiny_droplets(x),
        None => result
    };

    let result = match tiny_droplet_misses {
        Some(x) => result.tiny_droplet_misses(x),
        None => result
    };

    let result = match passed_objects {
        Some(x) => result.passed_objects(x),
        None => result
    };

    let result = match nmiss {
        Some(x) => result.misses(x),
        None => result.misses(0)
    };

    let stats = CatchResults {
        total_stars: map.stars(mods, None).stars(),
        partial_stars: map.stars(mods, passed_objects).stars(),
        pp: result.calculate().pp(),
    };

    return stats;
}

fn calculate_std_pp_py(_: Python, map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, potential_acc: Option<f32>, n300: Option<usize>,
                       n100: Option<usize>, n50: Option<usize>, nmiss: Option<usize>, passed_objects: Option<usize>) -> PyResult<StdResults> {
    let pp_stats = calculate_std_pp(map, mods, combo, acc, potential_acc, n300, n100, n50, nmiss, passed_objects);
    Ok(pp_stats)
}

fn calculate_taiko_pp_py(_: Python, map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, n300: Option<usize>,
                         n100: Option<usize>, nmiss: Option<usize>, passed_objects: Option<usize>) -> PyResult<TaikoResults> {
    let pp_stats = calculate_taiko_pp(map, mods, combo, acc,  n300, n100, nmiss, passed_objects);
    Ok(pp_stats)
}

fn calculate_mania_pp_py(_: Python, map: String, mods: u32, score: Option<u32>, passed_objects: Option<usize>) -> PyResult<ManiaResults> {
    let pp_stats = calculate_mania_pp(map, mods, score, passed_objects);
    Ok(pp_stats)
}

fn calculate_catch_pp_py(_: Python, map: String, mods: u32, combo: Option<usize>, fruits: Option<usize>,
                         droplets: Option<usize>, tiny_droplets: Option<usize>,
                         tiny_droplet_misses: Option<usize>, nmiss: Option<usize>,
                         passed_objects: Option<usize>) -> PyResult<CatchResults> {
    let pp_stats = calculate_catch_pp(map, mods, combo, fruits, droplets, tiny_droplets, tiny_droplet_misses, nmiss, passed_objects);
    Ok(pp_stats)
}