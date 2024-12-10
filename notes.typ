#set page(
  paper: "a4",
  margin: (x: 1cm, y: 1cm),
)
#set text(
  font: "Helvetica",
  size: 12pt
)

#set heading(numbering: "1:")

= Historian Hysteria
Getting back into this with Fox and Lexi because they started doing it on December 1st as well. Last year, Des did it as well, but none of us finished because, well, holidays are busy. i doubt that i will finish this year, but, hey, it gave me a good excuse to write more Rust, which i've been saying i wanted to do for a minute.

This was mostly straightforward, though the two parts of the problem don't really cleanly build on each other, since part one wants the two lists zipped and consumed and part two wants a count of elements from one list without consuming it. Nothing that weird though.

i have nevertheless been trying to make sure that i don't break the "part one" answers when i write the part twos, because that really sucked last year.

= Red-Nosed Reports
Also pretty straightforward. Part two is doable entirely in terms of part one, so i just solve part two by checking if it's safe under the rules of part one and, if it's not, removing numbers one by one until *any* of those lists is safe under the rules of part one, which accounts for the lists covered by the Problem Dampener™.

`is_safe_one` could probably be written in terms of `take_while` to be more idiomatic.

= Mull It Over
Well, this one was really easy, but it was finally time for me to figure out how to set up Cargo because I needed the `regex` crate. So that made it take longer than it otherwise would have (I don't actually know what Cargo's fairly-opinionated file structure wants, and had to cat up and learn that here).

I am a bit surprised that there's no regex support in the standard library?

Part two was easy once I realized that I don't need to track enabled/disabled states because I can just nuke the disabled instructions from the input entirely.

= Ceres Search
Okay, the pun in part two here is extremely stupid but funny.

I have often heard that usize/i32 conversion can be a real pain in Rust, but I'm seeing it live here. The rest of this is mostly just about making this as FP-shaped as possible. There's no directionality to part two: anything on the outer edge of the box will _never_ be an X-MAS. So part two ends up just being about enumerating the four possible shapes you could have. I'm sure there's a "cooler" way to do that, though.

= Print Queue
Writing `is_correctly_ordered` here turned out to be very useful, because then it allowed me to have the bad lists just be `!is_correctly_ordered` when part two rolled around.

I had originally reordered by swapping the two elements that were in the wrong order, but later opted to switch to "move the wrong one immediately after the right one", which turns out to be a fair bit faster, though I think this is also just because of the nature of the inputs.

I had to level up my Rust FP skills a bit to write `main()` without like eight different lists, but that was rewarding~

= Guard Gallivant
The elephant in the room: this code is *grievously* slow, but I don't need to be putting any more time into it on a work day. :) May revisit it later though.

The funniest bug here was that I was, at one point, starting the loop checker from the position of the newly-placed obstacle rather than the guard, and at one point in my inputs this just so happens to put the guard into an infinite loop of turning right. Oops. Sorry, Guard-friend.

= Bridge Repair
A lot of this problem was just me combinatorics-ing too close to the sun. I had initially written the permutation finder in a way that it generated 2^n permutations instead of 2 << n.

I also tried to be clever and use `meval` for part one, but it just wants to obey PEMDAS a bit too much for that. :) This turned out to be useful because part two would require it anyway.

= Resonant Collinearity

So I ended up drawing a TON of random pictures to make sure my algorithm even made sense here, which I'll share here for funsies.

```
a1r > a2r and a1c > a2c: (3,3), (2,2)
antinodes (1,1) and (4,4)
......
.a....
..2...
...1..
....a.
......
```

```
a1r < a2r and a1c > a2c: (2,3), (3,2)
antinodes (1,4) and (4,1)
......
....a.
...1..
..2...
.a....
......
```

```
a1r > a2r and a1c < a2c: (3,2), (2,3)
antinodes (4,1) and (1,4)
......
....a.
...2..
..1...
.a....
......
```

```
a1r < a2r and a1c < a2c: (2,2), (3,3)
antinodes (4,4) and (1,1)
......
.a....
..1...
...2..
....a.
......
```

and the slightly-special cases:
```
a1c == a2c: (3,1), (2,1)
antinodes (4,1) and (1,1)
......
.a....
.2....
.1....
.a....
......
```

```
a1r == a2r: (1,2), (1,3)
antinodes (1,1) and (1,4)
......
.a12a.
......
......
......
......
```

so what covers all of these?
```
dr = a1r - a2r
dc = a1c - a2c
anti1: (a1r + dr, a1c + dc)
anti2: (a2r - dr, a2c - dc)
```

```
(3,3), (2,2) -> dr: 1, dc: 1 (3+1 3+1) and (2-1 2-1) 
(2,3), (3,2) -> dr: -1, dc: 1 (2+-1 3+1) and (3--1 2-1) 
(3,2), (2,3) -> dr: 1, dc: -1 (3+1 2+-1) and (2-1 3--1) 
(2,2), (3,3) -> dr: -1, dc: -1 (2+-1 2+-1) and (3--1 3--1) 
(3,1), (2,1) -> dr: 1, dc: 0 (3+1 1+0) and (2-1 1-0) 
(2,1), (3,1) -> dr: -1, dc: 0 (2+-1 1+0) and (3--1 1-0) 
(1,3), (1,2) -> dr: 0, dc: 1 (1+0 3+1) and (1+0 2-1) 
(1,2), (1,3) -> dr: 0, dc: -1 (1+0 2+-1) and (1+0 3--1) 
```

Anyway, uh, that was fun. Part two took me an embarassingly long time to realize that the antennae, themselves, are also nodes, but is otherwise almost identical to part one.

= Disk Fragmenter

I found this one honestly kinda overwhelming to *read* but it was fun to implement. 

I pity the people who decided to just keep things as chars and then realized "oh wait IDs can be > 9 huh"; one person's solution on Reddit for this one uses emoji for everything over 9 and I *do* respect the utter commitment to the bit.

= Hoof It

I suppose we were overdue for the obligatory bfs/dfs question this year. 

I accidentally solved part two before part one here, but that also kinda seems like the more "natural" bfs question. I went back and added a hashset to only track unique peaks instead of all paths, then I just hit undo a few times. At that point it annoyed me how un-DRY my code was so I just smashed `bfs_one` and `bfs_two` into each other and had them return a tuple.
