use std::collections::HashMap;

fn main() {
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

    let university_names = universities.keys();

    // let mut permutations = [];

    let n: usize = university_names.len();

    println!("{}", n);
    let mut p: [i32; 11] = [10i32; 11];
    let mut i: i32 = 0;

    let mut shortest_trip: i32 = 1000000;
    let mut shortest_trip_universities: [char; 10] = [""; 10];

    while i < n as i32 {
        p[i] = p[i] - 1;

        let j: i32 = i % 2 * p[i];

        // $tmp = $university_names[$i];
        // $university_names[$i] = $university_names[$j];
        // $university_names[$j] = $tmp;
        //
        // $cities = &university_names;
        // $trip_length = 0;
        // for ($i = 0; $i < count($cities) - 1; $i++) {
        //     $lat1 = $universities[$cities[$i + 1]]["lat"];
        //     $lat0 = $universities[$cities[$i]]["lat"];
        //     $long1 = $universities[$cities[$i + 1]]["long"];
        //     $long0 = $universities[$cities[$i]]["long"];
        //     $trip_length += sqrt(($lat1 - $lat0)*($lat1 - $lat0) + ($long1 - $long0)*($long1 - $long0));
        // }
        //
        // if ($trip_length < $shortest_trip) {
        //     $shortest_trip = $trip_length;
        //     $shortest_trip_universities = $cities;
        // }
        //
        // $i = 1;
        // while ($p[$i] == 0) {
        //     $p[$i] = $i;
        //     $i++;
        // }
    }
    //
    // $shortest_trip = PHP_INT_MAX;
    // $shortest_trip_universities = [];
    //
    // foreach ($permutations as $key => $trip) {
    //     $trip_length = 0;
    //
    //     for ($i = 0; $i < count($trip) - 1; $i++) {
    //         $lat1 = $universities[$trip[$i + 1]]["lat"];
    //         $lat0 = $universities[$trip[$i]]["lat"];
    //         $long1 = $universities[$trip[$i + 1]]["long"];
    //         $long0 = $universities[$trip[$i]]["long"];
    //         $trip_length += sqrt(($lat1 - $lat0)*($lat1 - $lat0) + ($long1 - $long0)*($long1 - $long0));
    //     }
    //
    //     if ($trip_length < $shortest_trip) {
    //         $shortest_trip = $trip_length;
    //         $shortest_trip_universities = $trip;
    //     }
    // }
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

}
