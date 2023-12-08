# Day 8: Haunted Wasteland

[Check it out on adventofcode.com](https://adventofcode.com/2023/day/8)

## Part One

You're still riding a camel across Desert Island when you spot a sandstorm quickly approaching. When you turn to warn the Elf, she disappears before your eyes! To be fair, she had just finished warning you about _ghosts_ a few minutes ago.

One of the camel's pouches is labeled "maps" - sure enough, it's full of documents (your puzzle input) about how to navigate the desert. At least, you're pretty sure that's what they are; one of the documents contains a list of left/right instructions, and the rest of the documents seem to describe some kind of _network_ of labeled nodes.

It seems like you're meant to use the _left/right_ instructions to _navigate the network_. Perhaps if you have the camel follow the same instructions, you can escape the haunted wasteland!

After examining the maps for a bit, two nodes stick out: `AAA` and `ZZZ`. You feel like `AAA` is where you are now, and you have to follow the left/right instructions until you reach `ZZZ`.

This format defines each _node_ of the network individually. For example:

    RL
    
    AAA = (BBB, CCC)
    BBB = (DDD, EEE)
    CCC = (ZZZ, GGG)
    DDD = (DDD, DDD)
    EEE = (EEE, EEE)
    GGG = (GGG, GGG)
    ZZZ = (ZZZ, ZZZ)
    

Starting with `AAA`, you need to _look up the next element_ based on the next left/right instruction in your input. In this example, start with `AAA` and go _right_ (`R`) by choosing the right element of `AAA`, `_CCC_`. Then, `L` means to choose the _left_ element of `CCC`, `_ZZZ_`. By following the left/right instructions, you reach `ZZZ` in `_2_` steps.

Of course, you might not find `ZZZ` right away. If you run out of left/right instructions, repeat the whole sequence of instructions as necessary: `RL` really means `RLRLRLRLRLRLRLRL...` and so on. For example, here is a situation that takes `_6_` steps to reach `ZZZ`:

    LLR
    
    AAA = (BBB, BBB)
    BBB = (AAA, ZZZ)
    ZZZ = (ZZZ, ZZZ)
    

Starting at `AAA`, follow the left/right instructions. _How many steps are required to reach `ZZZ`?_

Your puzzle answer was `19631`.

## Part Two

The sandstorm is upon you and you aren't any closer to escaping the wasteland. You had the camel follow the instructions, but you've barely left your starting position. It's going to take _significantly more steps_ to escape!

What if the map isn't for people - what if the map is for _ghosts_? Are ghosts even bound by the laws of spacetime? Only one way to find out.

After examining the maps a bit longer, your attention is drawn to a curious fact: the number of nodes with names ending in `A` is equal to the number ending in `Z`! If you were a ghost, you'd probably just _start at every node that ends with `A`_ and follow all of the paths at the same time until they all simultaneously end up at nodes that end with `Z`.

For example:

    LR
    
    11A = (11B, XXX)
    11B = (XXX, 11Z)
    11Z = (11B, XXX)
    22A = (22B, XXX)
    22B = (22C, 22C)
    22C = (22Z, 22Z)
    22Z = (22B, 22B)
    XXX = (XXX, XXX)
    

Here, there are two starting nodes, `11A` and `22A` (because they both end with `A`). As you follow each left/right instruction, use that instruction to _simultaneously_ navigate away from both nodes you're currently on. Repeat this process until _all_ of the nodes you're currently on end with `Z`. (If only some of the nodes you're on end with `Z`, they act like any other node and you continue as normal.) In this example, you would proceed as follows:

*   Step 0: You are at `11A` and `22A`.
*   Step 1: You choose all of the _left_ paths, leading you to `11B` and `22B`.
*   Step 2: You choose all of the _right_ paths, leading you to `_11Z_` and `22C`.
*   Step 3: You choose all of the _left_ paths, leading you to `11B` and `_22Z_`.
*   Step 4: You choose all of the _right_ paths, leading you to `_11Z_` and `22B`.
*   Step 5: You choose all of the _left_ paths, leading you to `11B` and `22C`.
*   Step 6: You choose all of the _right_ paths, leading you to `_11Z_` and `_22Z_`.

So, in this example, you end up entirely on nodes that end in `Z` after `_6_` steps.

Simultaneously start on every node that ends with `A`. _How many steps does it take before you're only on nodes that end with `Z`?_

Your puzzle answer was `21003205388413`
