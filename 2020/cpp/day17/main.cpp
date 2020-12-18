#include "../utils.h"
#include "grid.h"

#include <chrono>

const std::vector<std::tuple<int, int, int>> directions = {
        {1,1, 0}, {1,0, 0}, {1,-1, 0}, {0,1, 0}, {0,-1, 0}, {-1,1, 0}, {-1,0, 0}, {-1,-1, 0},
        {1,1, 1}, {1,0, 1}, {1,-1, 1}, {0,1, 1}, {0,-1, 1}, {-1,1, 1}, {-1,0, 1}, {-1,-1, 1}, {0,0, 1},
        {1,1,-1}, {1,0,-1}, {1,-1,-1}, {0,1,-1}, {0,-1,-1}, {-1,1,-1}, {-1,0,-1}, {-1,-1,-1}, {0,0,-1}
};

void gameOfLife(Grid3D& grid3d) {

  // RULES PART 1
  // 1. If a cube is active and exactly 2 or 3 of its neighbors are also active,
  //    the cube remains active. Otherwise, the cube becomes inactive.
  // 2. If a cube is inactive but exactly 3 of its neighbors are active,
  //    the cube becomes active. Otherwise, the cube remains inactive.

  for (int cycle = 0; cycle < 6; cycle++) {
    // add inactive planes on the outside of the grid
    // needed for expanding into 3D
    {
      auto [minX, maxX, minY, maxY, minZ, maxZ] = grid3d.xyzBounds();
      for (int z = minZ-1; z <= maxZ+1; z++) {
        for (int x = minX-1; x <= maxX+1; x++) {
          for (int y = minY-1; y <= maxY+1; y++) {
            if (z >= minZ && z <= maxZ) {
              if (x >= minX && x <= maxX && y >= minY && y <= maxY) continue;
              grid3d.inactive_.insert(Coord3D(x, y, z));
            } else {
              grid3d.inactive_.insert(Coord3D(x, y, z));
            }
          } // y
        } // x
      } // z
    }
    Grid3D tmp(grid3d);
    grid3d.clear();

    // play
    for (const auto& coord : tmp.active_) {
      int count = 0;
      for (const auto& [dx, dy, dz] : directions) {
        Coord3D adjacent(coord.x_[0]+dx,coord.x_[1]+dy,coord.x_[2]+dz);
        if (tmp.active_.find(adjacent) != tmp.active_.end()) {
          count++;
        }
      } // directions

      if (count == 2 || count == 3) {
        grid3d.active_.insert(coord);
      } else {
        grid3d.inactive_.insert(coord);
      }
    } // coords active

    for (const auto& coord : tmp.inactive_) {
      int count = 0;
      for (const auto& [dx, dy, dz] : directions) {
        Coord3D adjacent(coord.x_[0]+dx,coord.x_[1]+dy,coord.x_[2]+dz);
        if (tmp.active_.find(adjacent) != tmp.active_.end()) count++;
      } // directions
      if (count == 3) {
        grid3d.active_.insert(coord);
      } else {
        grid3d.inactive_.insert(coord);
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

 
} 
