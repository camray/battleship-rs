# battleship-rs

I'm using this project to learn Rust.

## TODO
- [ ] Render Player and Enemy version of each board
- [ ] Player turn
    - Each player takes turns firing at the other player's board
    - Players can only fire at un-fired-at locations
    - Players can only fire at locations on the board
- [ ] Fire at a location on the board
    - If the player hits a ship, mark the ship as a hit
    - Mark the space as a hit
- [ ] After players turn, determine
    - If the player hit a ship
    - If the player sunk a ship
    - If the player won the game
- [ ] Game over

## Done
- [x] Create a board
    - Board is a 10x10 grid
    - Board is initialized with all empty spaces
- [x] Create a ship
- [x] Place a ship on the board
- [x] Basic interaction rules are set up
- [x] Initial player setup
    - Each player takes turns placing their ships on the board
    - Ships are placed horizontally or vertically
    - Ships cannot overlap
    - Ships cannot be placed off the board
