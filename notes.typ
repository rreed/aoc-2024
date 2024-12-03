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
