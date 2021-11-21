use std::fs::File;
use rosu_pp::Beatmap;

pub(crate) fn parse_map(file_path: String) -> Beatmap {
    let file = match File::open(file_path) {
        Ok(file) => file,
        Err(why) => panic!("Could not open file: {}", why),
    };

    return match Beatmap::parse(file) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };
}