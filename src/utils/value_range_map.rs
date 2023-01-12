pub fn map_range_fromxy_toxy(
    value: f32,
    from_min: f32,
    from_max: f32,
    to_min: f32,
    to_max: f32,
) -> f32 {
    let from_range = from_max - from_min;
    let to_range = to_max - to_min;
    let value_scaled = (value - from_min) / from_range;
    to_min + (value_scaled * to_range)
}
