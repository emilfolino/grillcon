extern crate time;

use time::PreciseTime;

use std::collections::HashMap;

fn main() {
    let start = PreciseTime::now();

    let mut universities = HashMap::new();
    universities.insert("BTH", (56.18, 15.59));
    universities.insert("Uppsala", (59.85, 17.63));
    universities.insert("Lund", (55.71, 13.20));
    universities.insert("KTH", (59.34, 18.07));
    universities.insert("Chalmers", (57.68, 11.97));
    universities.insert("Luleå", (65.61, 22.14));
    universities.insert("Umeå", (63.82, 20.30));
    universities.insert("Linköping", (58.39, 15.57));
    universities.insert("Karlstad", (59.40, 13.58));
    universities.insert("Örebro", (59.25, 15.24));
    universities.insert("Linné", (56.85, 15.24));

    let mut university_names: Vec<String> = vec!["BTH".to_string(), "Uppsala".to_string(), "Lund".to_string()];
    let n: usize = university_names.len();

    let mut p = vec![n; n + 1];
    let mut i: usize = 0;

    let mut shortest_trip: i32 = 1000000;
    let mut shortest_trip_universities: Vec<String> = Vec::with_capacity(n as usize);

    while i < n {
        p[i] = p[i] - 1;

        let j: usize = i % 2 * p[i];

        university_names.swap(i, j);

        let cities = university_names.clone();
        let mut trip_length: i32 = 0;
        for k in 0..(cities.len() - 1) {
            // let lat1: f64 = universities[cities[k + 1].to_string()].0;
            // let lat0: f64 = universities[cities[k].to_string()].0;
            // let long1: f64 = universities[cities[k + 1].to_string()].1;
            // let long0: f64 = universities[cities[k].to_string()].1;
            // trip_length += sqrt((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0));
        }

        // if ($trip_length < $shortest_trip) {
        //     $shortest_trip = $trip_length;
        //     $shortest_trip_universities = $cities;
        // }

        i = 1;
        while p[i] == 0 {
            p[i] = i;
            i += 1;
        }
    }

    //
    // $time_end = microtime(true);
    //
    // $execution_time = ($time_end - $time_start);
    //
    //
    //
    // echo "Shortest trip length: " . $shortest_trip . "\n";
    // echo implode(" --> ", $shortest_trip_universities) . "\n";
    // echo "Total Execution Time: ".$execution_time." seconds\n";



    let end = PreciseTime::now();

    println!("{} seconds for.", start.to(end));
}
