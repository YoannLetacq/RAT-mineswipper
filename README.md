# Minesweeper in Rust

Welcome to Minesweeper in Rust! This project is a simple implementation of the classic Minesweeper game using the Rust programming language.

## Table of Contents

- [Introduction](#introduction)
- [Features](#features)
- [How to Play](#how-to-play)
- [Building and Running](#building-and-running)
- [Known Issues](#known-issues)

## Introduction

Minesweeper is a classic single-player puzzle game where the player must uncover hidden mines on a grid while avoiding detonating any of them. It's a great way to challenge your logic and deduction skills.

## Features

- **Game Grid**: A grid of cells where you can uncover tiles.
- **Mines**: Hidden mines are scattered across the grid.
- **Numbers**: Numbers indicate how many mines are adjacent to a cell.
- **Flagging**: You can flag cells that you suspect contain mines.
- **Winning**: The game is won when all non-mined cells are uncovered. not implemented yet
- **Losing**: The game is lost if you uncover a mine.
- **Timer**: Keep track of your time to complete the game. not implemented yet 
- **Question Mark**: Mark cells with a question mark when you're unsure. not implemented yet

## How to Play

1. Clone this repository to your local machine.
2. Open your terminal and navigate to the project directory.
3. Build the game using `cargo build`.
4. Run the game with `cargo run`.

Here are some basic controls:

- Left-click or A to uncover a cell.
- Right-click or E to flag a cell.
- Middle-click or Z to uncover all cells around a uncover cell .
- The game will display your time, mine count, and the status of the game. that actualy a lie but i will make it true

## Building and Running

To build and run the Minesweeper game, follow these steps:

1. Make sure you have Rust and Cargo installed on your system.
2. Clone this repository to your local machine using `git clone`.
3. Open a terminal and navigate to the project directory.
4. Build the game with the following command:
- `cargo build --release`
- move setting.txt and assets/ to the application 
5. Or run the code with `cargo run`


## Known Issues

- No winning screen
- No question mark case
- No mine count and no timer
- Some optimization 


Thank you for playing Minesweeper in Rust!

Happy mining!