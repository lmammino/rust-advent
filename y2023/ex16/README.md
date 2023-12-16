# Day 16: The Floor Will Be Lava

[Check it out on adventofcode.com](https://adventofcode.com/2023/day/16)

## Part One

With the beam of light completely focused _somewhere_, the reindeer leads you deeper still into the Lava Production Facility. At some point, you realize that the steel facility walls have been replaced with cave, and the doorways are just cave, and the floor is cave, and you're pretty sure this is actually just a giant cave.

Finally, as you approach what must be the heart of the mountain, you see a bright light in a cavern up ahead. There, you discover that the beam of light you so carefully focused is emerging from the cavern wall closest to the facility and pouring all of its energy into a contraption on the opposite side.

Upon closer inspection, the contraption appears to be a flat, two-dimensional square grid containing _empty space_ (`.`), _mirrors_ (`/` and `\`), and _splitters_ (`|` and `-`).

The contraption is aligned so that most of the beam bounces around the grid, but each tile on the grid converts some of the beam's light into _heat_ to melt the rock in the cavern.

You note the layout of the contraption (your puzzle input). For example:

    .|...\....
    |.-.\.....
    .....|-...
    ........|.
    ..........
    .........\
    ..../.\\..
    .-.-/..|..
    .|....-|.\
    ..//.|....
    

The beam enters in the top-left corner from the left and heading to the _right_. Then, its behavior depends on what it encounters as it moves:

*   If the beam encounters _empty space_ (`.`), it continues in the same direction.
*   If the beam encounters a _mirror_ (`/` or `\`), the beam is _reflected_ 90 degrees depending on the angle of the mirror. For instance, a rightward-moving beam that encounters a `/` mirror would continue _upward_ in the mirror's column, while a rightward-moving beam that encounters a `\` mirror would continue _downward_ from the mirror's column.
*   If the beam encounters the _pointy end of a splitter_ (`|` or `-`), the beam passes through the splitter as if the splitter were _empty space_. For instance, a rightward-moving beam that encounters a `-` splitter would continue in the same direction.
*   If the beam encounters the _flat side of a splitter_ (`|` or `-`), the beam is _split into two beams_ going in each of the two directions the splitter's pointy ends are pointing. For instance, a rightward-moving beam that encounters a `|` splitter would split into two beams: one that continues _upward_ from the splitter's column and one that continues _downward_ from the splitter's column.

Beams do not interact with other beams; a tile can have many beams passing through it at the same time. A tile is _energized_ if that tile has at least one beam pass through it, reflect in it, or split in it.

In the above example, here is how the beam of light bounces around the contraption:

    >|<<<\....
    |v-.\^....
    .v...|->>>
    .v...v^.|.
    .v...v^...
    .v...v^..\
    .v../2\\..
    <->-/vv|..
    .|<<<2-|.\
    .v//.|.v..
    

Beams are only shown on empty tiles; arrows indicate the direction of the beams. If a tile contains beams moving in multiple directions, the number of distinct directions is shown instead. Here is the same diagram but instead only showing whether a tile is _energized_ (`#`) or not (`.`):

    ######....
    .#...#....
    .#...#####
    .#...##...
    .#...##...
    .#...##...
    .#..####..
    ########..
    .#######..
    .#...#.#..
    

Ultimately, in this example, `_46_` tiles become _energized_.

The light isn't energizing enough tiles to produce lava; to debug the contraption, you need to start by analyzing the current situation. With the beam starting in the top-left heading right, _how many tiles end up being energized?_

Your puzzle answer was `8034`.

## Part Two

As you try to work out what might be wrong, the reindeer tugs on your shirt and leads you to a nearby control panel. There, a collection of buttons lets you align the contraption so that the beam enters from _any edge tile_ and heading away from that edge. (You can choose either of two directions for the beam if it starts on a corner; for instance, if the beam starts in the bottom-right corner, it can start heading either left or upward.)

So, the beam could start on any tile in the top row (heading downward), any tile in the bottom row (heading upward), any tile in the leftmost column (heading right), or any tile in the rightmost column (heading left). To produce lava, you need to find the configuration that _energizes as many tiles as possible_.

In the above example, this can be achieved by starting the beam in the fourth tile from the left in the top row:

    .|<2<\....
    |v-v\^....
    .v.v.|->>>
    .v.v.v^.|.
    .v.v.v^...
    .v.v.v^..\
    .v.v/2\\..
    <-2-/vv|..
    .|<<<2-|.\
    .v//.|.v..
    

Using this configuration, `_51_` tiles are energized:

    .#####....
    .#.#.#....
    .#.#.#####
    .#.#.##...
    .#.#.##...
    .#.#.##...
    .#.#####..
    ########..
    .#######..
    .#...#.#..
    

Find the initial beam configuration that energizes the largest number of tiles; _how many tiles are energized in that configuration?_

Your puzzle answer was `8225`.
