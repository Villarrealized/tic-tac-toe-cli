# Tic-Tac-Toe
The goal of tic-tac-toe is to be the first player to place 3
of your marks in a horizontal, vertical, or diagonal line.

When it is your turn, enter the number of the cell you wish to mark.

- Type `q`, `quit`, or `exit` to quit the game.
- Type `r`, `reset`, or `restart` to start over.
- Type `l` or `toggle_layout` to toggle between the numbered and blank layouts
- Type `c` or `toggle_color` to show X's and O's colored or plain

#### Examples
```
O Wins         X Wins        Draw
   |   |          |   |          |   |      
 O |   | X        |   | O      O | X | O 
___|___|___    ___|___|___    ___|___|___
   |   |          |   |          |   |    
 X | O | X      X | X | X      O | X | O 
---|---|---    ---|---|---    ---|---|--- 
   |   | O        |   | O      X | O | X
   |   |          |   |          |   |   
```

Run `cargo run` to play the game, or download the binary from the [releases](https://github.com/Villarrealized/tic-tac-toe-cli/releases) page

![](https://user-images.githubusercontent.com/5977736/83929730-a6273b80-a751-11ea-8358-40f069b7f836.png)