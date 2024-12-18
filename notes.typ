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

= Plutonian Pebbles

As I write, I can hear the Outer Wilds space banjo.

I honestly really enjoy the problems like this, they're really fun. :) There's probably a fancy flat_mappy way to do this.

This is the typical AOC problem of "do something with a reasonable input then do that same thing with an unreasonable one". Since the input gets 50% larger (roughly) each time you need to avoid anything that scales too fast.

= Garden Groups

*Oof.*

I ended up throwing away an entire close-to-right solution here (i.e., it worked for part one but part two was overcounting just a bit in ways I never debugged). I had originally done `Vec<Vec<char>>` for the return type of `read_input` as I've done in every other similar problem, but I decided that I was extremely tired of converting back and forth between isize and usize dozens of times so I just ended up padding out with `.` as a sentinel value. 

The realization of "the number of corners in a polygon is also the number of sides in a polygon" helped a ton~

= Claw Contraption

I read this, saw that Alex hadn't gotten part two yet, and said "oh no, part two is going to just be another insane input that hobbles my algorithm again, huh". 

I didn't start with the linear algebra stuff, *hoping* that I wouldn't have to, solving part one with a gcd helper and BFS (again)..but then I ended up having to just scrap part one to do linear algebra because part two was just "add 1e13 to both targets" and the recursive thing just WAS NOT going to work. 

Anyway, I'm glad that I could just read about the math behind this because this problem is almost purely math, no "algorithm".

= Restroom Redoubt

Not feeling part two of this one *at all*. The intuitive leap to "they'll probably group up at some point to make the shape, and that means either they'll all be in the middle or all in one quadrant and thus the security score will be really low" is like...okay, fine, but that's not defined in the problem very well, so it just feels like "figure out the One Weird Trick" instead of an actual algorithm.

Part one was fun though~

= Warehouse Woes

Guh. This one was *tough* for me.

I ended up throwing away a lot of my code between part one and two and deciding that I was really tired of not having a decent approach to as many "grid-based" questions as AOC seems to have. This was my opportunity to learn more of the Rust type system, but that also meant that I didn't finish this until like 6pm because I wrote and rewrote my solution several times to get a decent answer.

Sigh, oh well, I'm like, not *actually* being competitive about this.

= Reindeer Maze

`BORN TO DIJKSTRA / WORLD IS A FUCK / Kill Em All 1989 / I am trash cat`

Uh, anyway. Yeah part one is just Dijkstra. Part two is more interesting: building up paths and then retracing back to anywhere that might have an identical score. This uses the fact that there's literally no reason to ever "turn around" to just check the 90 degrees on either side of the reindeer, which is kinda nice. Overall, I spent way longer than I wanted on part two.

= Chronospatial Computer

Oh boy, now we're cooking.

Part one is pretty straightforward, though this is also the most comments I've left in any file I've written for AoC this year. I'm sure there are fancy ways to do this, but I liked just having the instructions all written out and commented like that.

Part two is *spicy*, but in a way that I enjoyed a lot more than 14-2 (Restroom Redoubt a few paragraphs back). Since B is only ever written to by `bxl` `bst` and `bxc`, and all of those are either XORs of what's already there or a thing that ends in `modulo 8`, this register never actually gets out of hand. It's also the only place `out` reads from, meaning that we'll only ever get a single digit at a time. So we can basically just build up a working program backwards, u8 by u8. (Get it? Reverse engineering? Har har har...)

Typing `struct TimeComputer` made me really happy.
