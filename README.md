# CardChapter: Ministry

That's right, this is CardChapter... again!

CardChapter is a lightweight web software intended to provide a kind of...
hybrid between "comic book" and "visual novel" that will look good
on your phone's screen.

## Ministry?
The original version of CardChapter lives at `https://cardchapter.com` (for now)
and has its repository at `https://github.com/cube-drone/cardchapter`.
It is ... a whole different software package,
although it is intended to solve a similar problem to this one.

This is the second release, running on a completely different underlying technology (Rust and Preact, mostly),
and started with the codename "Ministry" - so this is the Ministry edition of CardChapter.


## Get Started

Currently the only way to install CardChapter is with `cargo`.

```
> cargo install cardchapter
> mkdir content_directory
> cd content_directory
> cardchapter init
> vim content.yml
> cardchapter serve
```