import string

heights = {char: i for i, char in enumerate(string.ascii_lowercase)}
heights.update({"S": 0, "E": 25})

def BFSearch(graph, start, end):
    visited_vertices = []
    queue = []

    visited_vertices.append(start)
    queue.append((start,0))

    while queue:

        (n,depth) = queue.pop(0)

        if n == end:
            return depth

        # getting vertices adjacent to the vertex n which is dequed.
        for v in graph[n]:
            if v not in visited_vertices:
                queue.append((v,depth+1))
                visited_vertices.append(v)
    return float("inf")

def check_weight(point1, point2):
    return (heights[point2] - heights[point1] <= 1)

def get_adjacent(grid, x, y):
    adjacent = []
    for (o1,o2) in [(1,0),(-1,0),(0,1),(0,-1)]:
        if (y+o2 >= 0) and (y+o2 < len(grid[0])) and (o1+ x >= 0) and (o1 + x < len(grid)) and check_weight(grid[x][y], grid[x+o1][y+o2]):
            adjacent.append((x+o1, y+o2))
    return adjacent

with open("input.txt", "r") as f:
    grid = [list(x.strip()) for x in f.readlines()]
    adjacent_list = {}
    a_list = []
    for i, row in enumerate(grid):
        for j, char in enumerate(row):
            adjacent_list[(i,j)] = get_adjacent(grid, i, j)
            if char == 'S':
                start = (i, j)
            elif char == 'E':
                end = (i, j)
            elif char == 'a':
                a_list.append((i, j))

#part1 = BFSearch(adjacent_list, start, end)

part2 = [BFSearch(adjacent_list, key, end) for key in a_list]

print(min(part2))
