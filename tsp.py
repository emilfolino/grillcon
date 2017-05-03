from timeit import default_timer as timer

start = timer()

# universities = {
#     'BTH' : {"lat" : 56.18, "long" : 15.59},
#     'Uppsala' : {"lat" : 59.85, "long" : 17.63},
#     'Lund' : {"lat" : 55.71, "long" : 13.20},
#     'KTH' : {"lat" : 59.34, "long" : 18.07},
#     'Chalmers' : {"lat" : 57.68, "long" : 11.97},
#     'Luleå' : {"lat" : 65.61, "long" : 22.14},
#     'Umeå' : {"lat" : 63.82, "long" : 20.30},
#     'Linköping' : {"lat" : 58.39, "long" : 15.57},
#     'Karlstad' : {"lat" : 59.40, "long" : 13.58},
#     'Örebro' : {"lat" : 59.25, "long" : 15.24}
# };
#
# var university_names = Object.keys(universities);
# var permutations = {};
#
# var N = university_names.length;
# var p = Array(N + 1).fill(N);
# var i = 0;
#
# while (i < N) {
#     p[i]--;
#
#     j = i%2 * p[i];
#
#     var tmp = university_names[i];
#     university_names[i] = university_names[j];
#     university_names[j] = tmp;
#
#     permutations[university_names.join("")] = university_names.slice();
#     i = 1;
#     while (p[i] == 0) {
#         p[i] = i;
#         i++;
#     }
# }
#
# var shortest_trip = Number.MAX_SAFE_INTEGER ;
# var shortest_trip_universities = {};
#
# for (var trip in permutations) {
#     if (permutations.hasOwnProperty(trip)) {
#         var trip_length = 0;
#         for (var i = 0; i < permutations[trip].length - 1; i++) {
#             var lat1 = universities[permutations[trip][i + 1]]["lat"];
#             var lat0 = universities[permutations[trip][i]]["lat"];
#             var long1 = universities[permutations[trip][i + 1]]["long"];
#             var long0 = universities[permutations[trip][i]]["long"];
#             trip_length += Math.sqrt((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0));
#         }
#
#         if (trip_length < shortest_trip) {
#             shortest_trip = trip_length;
#             shortest_trip_universities = permutations[trip];
#         }
#     }
# }


end = timer()

# console.log("Shortest trip length: " + shortest_trip);
# console.log(shortest_trip_universities.join(" --> "));

print("Total Execution Time: " + str(end - start))
