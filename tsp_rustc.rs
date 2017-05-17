fn main() {
    let universities: &[(&str, f64, f64)] = &[
        ("BTH", 56.18, 15.59),
        ("Uppsala", 59.85, 17.63),
        ("Lund", 55.71, 13.20),
        ("KTH", 59.34, 18.07),
        ("Chalmers", 57.68, 11.97),
        ("Luleå", 65.61, 22.14),
        ("Umeå", 63.82, 20.30),
        ("Linköping", 58.39, 15.57),
        ("Karlstad", 59.40, 13.58),
        ("Örebro", 59.25, 15.24),
        ("Linné", 56.85, 15.24),
    ];

    let n = universities.len();

    let mut cities: Vec<_> = (0 .. n).collect();
    let mut shortest_trip = std::f64::INFINITY;
    let mut shortest_trip_universities = vec![];
    let mut p = vec![0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
    let mut i = 0;

    while i < n {
        p[i] -= 1;
        let j = i % 2 * p[i];
        cities.swap(i, j);

        let mut trip_length = 0.0;

        for pair in cities.windows(2) {
            let (_, lat0, long0) = universities[pair[0]];
            let (_, lat1, long1) = universities[pair[1]];

            trip_length += ((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0)).sqrt();
        }

        if trip_length < shortest_trip {
            shortest_trip = trip_length;
            shortest_trip_universities = cities.to_vec();
        }

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    println!("Shortest trip length: {}", shortest_trip);

    // join should work on lazy iterables too.
    println!("{}", shortest_trip_universities
                   .iter()
                   .map(|&i| universities[i].0)
                   .collect::<Vec<_>>()
                   .join(" --> "));
}
