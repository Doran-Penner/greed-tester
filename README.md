In the recent section of Reed College's intro CS class, there was a project for a game called Greed. For fun, I made this to try to test "all possible outcomes" for a given strategy of the game and determine how successful it is. However, it doesn't really work --- there are so many possibilities that I'm getting `usize` integer overflow on my 64-bit computer, and don't really know how to get around it. It's cool code, though, and my first time writing any kind of substantive Rust code, so I decided to share it anyways!

***

You can play around with this by modifying `TERMINATION_BOUND`, `WIN_SCORE`, and `student`. There are some small numbers for which this doesn't overflow, but not big enough to be useful. If you find a way to solve the problem or want to change anything, feel free to make a PR!
