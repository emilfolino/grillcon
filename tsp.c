#include <stdio.h>
#include <limits.h>
#include <math.h>
#include <string.h>

int main () {
    int n = 11;
    const char *universities[11] = {"BTH", "Uppsala", "Lund", "KTH", "Chalmers", "Luleå", "Umeå", "Linköping", "Karlstad", "Örebro", "Linné"};

    char *mut_universities[11] = {"BTH", "Uppsala", "Lund", "KTH", "Chalmers", "Luleå", "Umeå", "Linköping", "Karlstad", "Örebro", "Linné"};

    const double coordinates[11][2] = {{56.18, 15.59}, {59.85, 17.63}, {55.71, 13.20}, {59.34, 18.07}, {57.68, 11.97}, {65.61, 22.14}, {63.82, 20.30}, {58.39, 15.57}, {59.40, 13.58}, {59.25, 15.24}, {56.85, 14.82}};

    int p[12] = {0, 1, 2, 3, 4, 5, 6, 7, 8, 9, 10, 11};

    double shortest_trip = INT_MAX;
    char *shortest_trip_universities[11] = {"", "", "", "", "", "", "", "", "", "", ""};

    int i = 0;
    while (i < n) {
        p[i]--;

        int j = i%2 * p[i];

        char *tmp = mut_universities[i];
        mut_universities[i] = mut_universities[j];
        mut_universities[j] = tmp;

        double trip_length = 0.0;
        for (int k = 0; k < n - 1; k++) {
            char *city0 = mut_universities[k];
            char *city1 = mut_universities[k + 1];

            int index0 = -1;
            int index1 = -1;
            for (int l = 0; l < n; l++) {
                if (strcmp(universities[l], city0) == 0) {
                    index0 = l;
                }

                if (strcmp(universities[l], city1) == 0) {
                    index1 = l;
                }
            }

            double lat1 = coordinates[index1][0];
            double lat0 = coordinates[index0][0];
            double long1 = coordinates[index1][1];
            double long0 = coordinates[index0][1];

            trip_length += sqrt((lat1 - lat0)*(lat1 - lat0) + (long1 - long0)*(long1 - long0));
        }


        if (trip_length < shortest_trip) {
            shortest_trip = trip_length;
            for (int m = 0; m < n; m++) {
                shortest_trip_universities[m] = mut_universities[m];
            }
        }

        i = 1;
        while (p[i] == 0) {
            p[i] = i;
            i++;
        }
    }

    printf("Shortest trip length: %f\n", shortest_trip);
    for (int i = 0; i < n - 1; i++) {
        printf("%s --> ", shortest_trip_universities[i]);
    }
    printf("%s\n", shortest_trip_universities[n - 1]);
    printf("Execution time: \n");

    return 0;
}
