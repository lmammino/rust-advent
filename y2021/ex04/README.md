# Day 4: Giant Squid

[Check it out on adventofcode.com](https://adventofcode.com/2021/day/4)


## Part One

You're already almost 1.5km (almost a mile) below the surface of the ocean, already so deep that you can't see any sunlight. What you can see, however, is a giant squid that has attached itself to the outside of your submarine.

Maybe it wants to play bingo?

Bingo is played on a set of boards each consisting of a `5`x`5` grid of numbers. Numbers are chosen at random, and the chosen number is marked on all boards on which it appears. (Numbers may not appear on all boards.) If all numbers in any row or any column of a board are marked, that board wins. (Diagonals don't count.)

The submarine has a bingo subsystem to help passengers (currently, you and the giant squid) pass the time. It automatically generates a random order in which to draw numbers and a random set of boards (your puzzle input). For example:

```plain
7,4,9,5,11,17,23,2,0,14,21,24,10,16,13,6,15,25,12,22,18,20,8,19,3,26,1

22 13 17 11  0
 8  2 23  4 24
21  9 14 16  7
 6 10  3 18  5
 1 12 20 15 19

 3 15  0  2 22
 9 18 13 17  5
19  8  7 25 23
20 11 10 24  4
14 21 16 12  6

14 21 17 24  4
10 16 15  9 19
18  8 23 26 20
22 11 13  6  5
 2  0 12  3  7
```

After the first five numbers are drawn (`7`, `4`, `9`, `5`, and `11`), there are no winners, but the boards are marked as follows (shown here adjacent to each other to save space):

<pre><code>22 13 17 <strong>11</strong>  0         3 15  0  2 22        14 21 17 24  <strong>4</strong>
 8  2 23  <strong>4</strong> 24         <strong>9</strong> 18 13 17  <strong>5</strong>        10 16 15  <strong>9</strong> 19
21  <strong>9</strong> 14 16  <strong>7</strong>        19  8  <strong>7</strong> 25 23        18  8 23 26 20
 6 10  3 18  <strong>5</strong>        20 <strong>11</strong> 10 24  <strong>4</strong>        22 <strong>11</strong> 13  6  <strong>5</strong>
 1 12 20 15 19        14 21 16 12  6         2  0 12  3  <strong>7</strong>
</code></pre>


After the next six numbers are drawn (`17`, `23`, `2`, `0`, `14`, and `21`), there are still no winners:


<pre><code>22 13 <strong>17</strong> <strong>11</strong>  <strong>0</strong>         3 15  <strong>0</strong>  <strong>2</strong> 22        <strong>14</strong> <strong>21</strong> <strong>17</strong> 24  <strong>4</strong>
 8  <strong>2</strong> <strong>23</strong>  <strong>4</strong> 24         <strong>9</strong> 18 13 <strong>17</strong>  <strong>5</strong>        10 16 15  <strong>9</strong> 19
<strong>21</strong>  <strong>9</strong> <strong>14</strong> 16  <strong>7</strong>        19  8  <strong>7</strong> 25 <strong>23</strong>        18  8 <strong>23</strong> 26 20
 6 10  3 18  <strong>5</strong>        20 <strong>11</strong> 10 24  <strong>4</strong>        22 <strong>11</strong> 13  6  <strong>5</strong>
 1 12 20 15 19        <strong>14</strong> <strong>21</strong> 16 12  6         <strong>2</strong>  <strong>0</strong> 12  3  <strong>7</strong>
</code></pre>


Finally, `24` is drawn:

<pre><code>22 13 <strong>17</strong> <strong>11</strong>  <strong>0</strong>         3 15  <strong>0</strong>  <strong>2</strong> 22        <strong>14</strong> <strong>21</strong> <strong>17</strong> <strong>24</strong>  <strong>4</strong>
 8  <strong>2</strong> <strong>23</strong>  <strong>4</strong> <strong>24</strong>         <strong>9</strong> 18 13 <strong>17</strong>  <strong>5</strong>        10 16 15  <strong>9</strong> 19
<strong>21</strong>  <strong>9</strong> <strong>14</strong> 16  <strong>7</strong>        19  8  <strong>7</strong> 25 <strong>23</strong>        18  8 <strong>23</strong> 26 20
 6 10  3 18  <strong>5</strong>        20 <strong>11</strong> 10 <strong>24</strong>  <strong>4</strong>        22 <strong>11</strong> 13  6  <strong>5</strong>
 1 12 20 15 19        <strong>14</strong> <strong>21</strong> 16 12  6         <strong>2</strong>  <strong>0</strong> 12  3  <strong>7</strong>
</code></pre>


At this point, the third board wins because it has at least one complete row or column of marked numbers (in this case, the entire top row is marked: `14` `21` `17` `24` `4`).

The score of the winning board can now be calculated. Start by finding the sum of all unmarked numbers on that board; in this case, the sum is `188`. Then, multiply that sum by the number that was just called when the board won, `24`, to get the final score, `188` * `24` = `4512`.

To guarantee victory against the giant squid, figure out which board will win first. **What will your final score be if you choose that board?**

Your puzzle answer was `5685`.


## Part Two

On the other hand, it might be wise to try a different strategy: let the giant squid win.

You aren't sure how many bingo boards a giant squid could play at once, so rather than waste time counting its arms, the safe thing to do is to figure out which board will win last and choose that one. That way, no matter which boards it picks, it will win for sure.

In the above example, the second board is the last to win, which happens after `13` is eventually called and its middle column is completely marked. If you were to keep playing until this point, the second board would have a sum of unmarked numbers equal to `148` for a final score of `148` * `13` = `1924`.

**Figure out which board will win last. Once it wins, what would its final score be?**

Your puzzle answer was `21070`.
