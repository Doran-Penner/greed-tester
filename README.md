In the recent section of Reed College's intro CS class, there was a project for a game called Greed. For fun, I made this to try to test "all possible outcomes" for a given strategy of the game and determine how successful it is. This is not a perfect metric --- it calculates how many rounds until a given strategy has some percentage (default 95) chance of having won by then --- but it's still very useful.

***

You can play around with this by modifying `TERMINATION_BOUND`, `WIN_SCORE`, and `student`. If you want to contribute, feel free to make a PR!
