### morpion-rs

Yet another [Tic-Tac-Toe]() CLI game (in Rust) against the computer.
The player to start is randomly selected.
The computer player is using [min max algorithm]() so it shouldn't be able to loose.

Example game sequence :
```
$ cargo run

You are : O
Computer is : X
Player to start : O

[1] | [2] | [3]
---------------
[4] | [5] | [6]
---------------
[7] | [8] | [9]

O > 5

 X  | [2] | [3]
---------------
[4] |  O  | [6]
---------------
[7] | [8] | [9]

O > 8

 X  |  X  | [3]
---------------
[4] |  O  | [6]
---------------
[7] |  O  | [9]

O > 3

 X  |  X  |  O
---------------
[4] |  O  | [6]
---------------
 X  |  O  | [9]

O > 4

 X  |  X  |  O
---------------
 O  |  O  |  X
---------------
 X  |  O  | [9]

O > 9

Tie !!

 X  |  X  |  O
---------------
 O  |  O  |  X
---------------
 X  |  O  |  O

```
