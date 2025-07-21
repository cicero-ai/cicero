
struct DataPoint {
    cylinder_index: i32,
    coord: Coordinate
    density: f32,
    size: i32,
    sentiment: i32,
    folding_xaxis: Option<i32>,
    related_cylinders: HashMap<i32, Vec<i32>>,
    creation_time: DateTime<Utc>,
    last_active: DateTime<Utc>,
    influencers: Vec<i32>,
    event_data: <i32>
}

struct Coordinate {
    x: i32,
    y: HashMap<i32, i32>,
    z: i32
}



