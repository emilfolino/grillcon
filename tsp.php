<?php

set_time_limit(1200);

$universities = [
    'BTH' => ["lat" => 56.18, "long" => 15.59],
    'Uppsala' => ["lat" => 59.85, "long" => 17.63],
    'Lund' => ["lat" => 55.71, "long" => 13.20],
    'KTH' => ["lat" => 59.34, "long" => 18.07],
    'Chalmers' => ["lat" => 57.68, "long" => 11.97],
    'Luleå' => ["lat" => 65.61, "long" => 22.14],
    'Umeå' => ["lat" => 63.82, "long" => 20.30],
    'Linköping' => ["lat" => 58.39, "long" => 15.57],
    'Karlstad' => ["lat" => 59.40, "long" => 13.58],
    'Örebro' => ["lat" => 59.25, "long" => 15.24],
    'Linné' => ["lat" => 56.85, "long" => 14.82]
];

$university_names = array_keys($universities);

$N = count($university_names);
$p = [0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11];
$i = 0;
$shortest_trip = PHP_INT_MAX;
$shortest_trip_universities = [];
while ($i < $N) {
    $p[$i]--;

    $j = $i%2 * $p[$i];

    $tmp = $university_names[$i];
    $university_names[$i] = $university_names[$j];
    $university_names[$j] = $tmp;

    $cities = $university_names;
    $trip_length = 0;
    for ($k = 0; $k < count($cities) - 1; $k++) {
        $lat1 = $universities[$cities[$k + 1]]["lat"];
        $lat0 = $universities[$cities[$k]]["lat"];
        $long1 = $universities[$cities[$k + 1]]["long"];
        $long0 = $universities[$cities[$k]]["long"];
        $trip_length += sqrt(($lat1 - $lat0)*($lat1 - $lat0) + ($long1 - $long0)*($long1 - $long0));
    }

    if ($trip_length < $shortest_trip) {
        $shortest_trip = $trip_length;
        $shortest_trip_universities = $cities;
    }

    $i = 1;
    while ($p[$i] == 0) {
        $p[$i] = $i;
        $i++;
    }
}

echo "Shortest trip length: " . $shortest_trip . "\n";
echo implode(" --> ", $shortest_trip_universities) . "\n";
echo "Total Execution Time: \n";
