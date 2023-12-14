grid = """#.###.#..
#.###.#..
..##.#.##
#.####.#.
#####...#
#.##.##..
..##.#...
##...###.
.#....#.#
#..#...##
#..#.#.##"""

def find_mirror(grid):
    for r in range(1, len(grid)):
        above = grid[:r][::-1]
        below = grid[r:]

        above = above[:len(below)]
        below = above[:len(above)]

        print(above)
        print("---")
        print(below)

        if above == below:
            return r
        
    return 0

print(find_mirror(list(grid.splitlines())))
