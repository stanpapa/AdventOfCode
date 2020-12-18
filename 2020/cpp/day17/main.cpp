#include "../utils.h"
#include "grid.h"

#include <chrono>

template <class T>
void gameOfLife(T& grid) {

  // RULES
  // 1. If a cube is active and exactly 2 or 3 of its neighbors are also active,
  //    the cube remains active. Otherwise, the cube becomes inactive.
  // 2. If a cube is inactive but exactly 3 of its neighbors are active,
  //    the cube becomes active. Otherwise, the cube remains inactive.

  for (int cycle = 0; cycle < 6; cycle++) {
    T tmp(grid);
    grid.clear();
    // add inactive planes on the outside of the grid
    // needed for expanding into 3D
    tmp.extend();

    // play
    for (const auto& coord : tmp.active_) {
      int count = tmp.nNeighbours(coord);
      if (count == 2 || count == 3) {
        grid.active_.insert(coord);
      } else {
        grid.inactive_.insert(coord);
      }
    } // coords active

    for (const auto& coord : tmp.inactive_) {
      int count = tmp.nNeighbours(coord);
      if (count == 3) {
        grid.active_.insert(coord);
      } else {
        grid.inactive_.insert(coord);
      }
    } // coords inactive
  } // cycle
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);

  // part 1
  Grid3D grid3d;
  for (int i = 0; i < input.size(); i++) {
    for (int j = 0; j < input[0].size(); j++) {
      if (input[i][j] == '#') {
        grid3d.active_.insert(Coord3D(i,j,0));
      } else {
        grid3d.inactive_.insert(Coord3D(i,j,0));
      }
    }
  }
  auto started = std::chrono::high_resolution_clock::now();
  gameOfLife(grid3d);
  std::cout << "Answer Part 1: " << grid3d.nActive() << "\n";
  auto done = std::chrono::high_resolution_clock::now();
  std::cout << "Time elapsed: " << std::chrono::duration_cast<std::chrono::milliseconds>(done-started).count() << "ms\n";

  // part 2
  Grid4D grid4d;
  for (int i = 0; i < input.size(); i++) {
    for (int j = 0; j < input[0].size(); j++) {
      if (input[i][j] == '#') {
        grid4d.active_.insert(Coord4D(i,j,0,0));
      } else {
        grid4d.inactive_.insert(Coord4D(i,j,0,0));
      }
    }
  }
  started = std::chrono::high_resolution_clock::now();
  gameOfLife(grid4d);
  std::cout << "Answer Part 2: " << grid4d.nActive() << "\n";
  done = std::chrono::high_resolution_clock::now();
  std::cout << "Time elapsed: " << std::chrono::duration_cast<std::chrono::milliseconds>(done-started).count() << "ms\n";
 
} 
