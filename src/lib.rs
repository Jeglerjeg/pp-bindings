extern crate cpython;

use cpython::{py_fn, py_module_initializer, PyResult, Python};
mod pp;

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

fn calculate_std_pp_py(_: Python, map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, potential_acc: Option<f32>, n300: Option<usize>,
                       n100: Option<usize>, n50: Option<usize>, nmiss: Option<usize>, passed_objects: Option<usize>) -> PyResult<pp::osu::StdResults> {
    let pp_stats = pp::osu::calculate_std_pp(map, mods, combo, acc, potential_acc, n300, n100, n50, nmiss, passed_objects);
    Ok(pp_stats)
}

fn calculate_taiko_pp_py(_: Python, map: String, mods: u32, combo: Option<usize>, acc: Option<f32>, n300: Option<usize>,
                         n100: Option<usize>, nmiss: Option<usize>, passed_objects: Option<usize>) -> PyResult<pp::taiko::TaikoResults> {
    let pp_stats = pp::taiko::calculate_taiko_pp(map, mods, combo, acc,  n300, n100, nmiss, passed_objects);
    Ok(pp_stats)
}

fn calculate_mania_pp_py(_: Python, map: String, mods: u32, score: Option<u32>, passed_objects: Option<usize>) -> PyResult<pp::mania::ManiaResults> {
    let pp_stats = pp::mania::calculate_mania_pp(map, mods, score, passed_objects);
    Ok(pp_stats)
}

fn calculate_catch_pp_py(_: Python, map: String, mods: u32, combo: Option<usize>, fruits: Option<usize>,
                         droplets: Option<usize>, tiny_droplets: Option<usize>,
                         tiny_droplet_misses: Option<usize>, nmiss: Option<usize>,
                         passed_objects: Option<usize>) -> PyResult<pp::catch::CatchResults> {
    let pp_stats = pp::catch::calculate_catch_pp(map, mods, combo, fruits, droplets, tiny_droplets, tiny_droplet_misses, nmiss, passed_objects);
    Ok(pp_stats)
}