use std::collections::HashMap;

pub fn fetch_limit_offset_by_map(map: &HashMap<String, String>) -> (u32, u32) {
    let limit = map
        .get("size")
        .map_or(10, |v| v.parse::<u32>().unwrap_or(10));
    let offset = map
        .get("page")
        .map_or(0, |v| (v.parse::<u32>().unwrap_or(1) - 1) * limit);
    (limit, offset)
}
