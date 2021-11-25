use rosu_pp::Beatmap;

pub(crate) fn parse_map(file_path: String) -> Beatmap {
    return match Beatmap::from_path(file_path) {
        Ok(map) => map,
        Err(why) => panic!("Error while parsing map: {}", why),
    };
}
