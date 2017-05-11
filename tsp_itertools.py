from itertools import permutations
from timeit import default_timer as timer



def distance(point1, point2):
    """
    Returns the Euclidean distance of two points in the Cartesian Plane.

    >>> distance([3,4],[0,0])
    5.0
    >>> distance([3,6],[10,6])
    7.0
    """
    return ((point1[0] - point2[0])**2 + (point1[1] - point2[1])**2) ** 0.5



def total_distance(points):
    """
    Returns the length of the path passing throught
    all the points in the given order.

    >>> total_distance([[1,2],[4,6]])
    5.0
    >>> total_distance([[3,6],[7,6],[12,6]])
    9.0
    """
    return sum([distance(point, points[index + 1]) for index, point in enumerate(points[:-1])])



def travelling_salesman(points):
    """
    Finds the shortest route to visit all the cities by bruteforce.
    Time complexity is O(N!), so never use on long lists.
    """

    return min([perm for perm in permutations(points)], key=total_distance)



def main():
    ## Points are universities in Sweden
    ## Order is: BTH, Uppsala, Lund, KTH, Chalmers, Luleå, Umeå,
    ## Linköping, Karlstad, Örebro, Linné

    start = timer()
    points = [(56.18, 15.59), (59.85, 17.63), (55.71, 13.20),
                (59.34, 18.07), (57.68, 11.97), (65.61, 22.14),
                (63.82, 20.30), (58.39, 15.57), (59.40, 13.58), (59.25, 15.24), (56.85, 14.82)]
    print("""The minimum distance to visit all the following points: {}
        starting at {} is {}.""".format(
        tuple(points),
        points[0],
        total_distance(travelling_salesman(points))))

    end = timer()
    print("Timing: " + str(end - start))



if __name__ == "__main__":
    main()
