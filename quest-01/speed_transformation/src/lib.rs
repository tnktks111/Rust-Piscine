pub fn km_per_hour_to_meters_per_second(km_h: f64) -> f64 {
    let m_h = km_h * 1000.0;
    let m_m = m_h / 60.0;
    let m_s = m_m / 60.0;
    m_s
}
