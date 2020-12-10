We implemented the encoding part in Rust and the GUI part in Python.

1, test rust file
cd sat
first compile the rust files with "cargo run --release"
then execute "target/release/tents src/tents-25x20-t.txt", exchange "src/tents-25x20-t.txt" with any other text files of the tents puzzle. To check the uniqueness of the puzzle solution, just add "unique" as the second argument. If the solution is unique, the code will first print one solution and then print "UNSAT" (meaning that there is no other solution except the first one). We used "cadical-sc2020-45029f8/build/cadical" (in tents.rs) as the path for calling the cadical solver. Please change the path to your own solver. To visualise the results, please read the part for the GUI.

2, use the GUI
The existence of the rust executable "target/release/tents" is import for this GUI to work. So please first compile the rust files before trying the GUI. Pygame and numpy also must be installed

Use "python3 src/tents.py" to open the GUI menu. First you need to enter the puzzle size (default size is 8x8). Then you can either use "Create game manually" to freely put tents or trees or use "Generate random game" to generate puzzles automatically. 

You can also play the game after creating a game or using "Load game from file". You can change the path in the input field to specify your puzzle file.

In the play mode, if you click on "Solve", the rust executable will be called to encode the puzzle and solve it with cadical. The result will be displayed if there is a solution, otherwise "UNSAT" will be displayed.

For a description of our encoding and performance profiling, please read the performance.txt.


