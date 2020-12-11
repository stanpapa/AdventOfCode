#include "../utils.h"

bool changeSeat(const std::vector<std::string>& grid,
                     const int& i, const int& j, const int& part) {
  int adjacent = 0;
  int rows = grid.size();
  int columns = grid[0].size();

  const std::vector<std::pair<int, int>> directions = {{1,1}, {1,0}, {1,-1}, 
  {0,1}, {0,-1}, {-1,1}, {-1,0}, {-1,-1}};
  
  // RULES PART 1:
  // 1. If a seat is empty (L) and there are no occupied seats adjacent to it,
  //    the seat becomes occupied.
  // 2. If a seat is occupied (#) and four or more seats adjacent to it
  //    are also occupied, the seat becomes empty.
  // 3. Otherwise, the seat's state does not change.

  // RULES PART 2:
  // 1. If a seat is empty (L) and there are no occupied seats in each 
  // direction to it, the seat becomes occupied.
  // 2. If a seat is occupied (#) and five or more seats adjacent to it
  //    are also occupied, the seat becomes empty.
  // 3. Otherwise, the seat's state does not change.

  // count number of adjacent occupied seats
  if (part == 1) {
    // check all 8 adjacent seats
    for (const auto& [dx, dy] : directions) {
      int x = i + dx;
      int y = j + dy;

      if (x < 0 || x >= rows) continue;
      if (y < 0 || y >= columns) continue;
      if (grid[x][y] == '#') adjacent++;
    }
  } else if (part == 2) {
    // check all 8 directions
    for (const auto& [dx, dy] : directions) {
      int x = i + dx;
      int y = j + dy;

      // keep going in the same direction
      while ((x >= 0 && x < rows) && (y >= 0 && y < columns)) {
        if (grid[x][y] == 'L') break;
        if (grid[x][y] == '#') {
          adjacent++;
          break;
        }
        x += dx;
        y += dy;
      }
    }
  } else {
    std::cerr << "Invalid part! ABORT.\n";
    exit(10);
  }

  // depending on the rules we have to change the seat
  int max_adjacent = 4;
  if (part == 2) max_adjacent = 5;
  if ((grid[i][j] == 'L' && adjacent == 0) || 
      (grid[i][j] == '#' && adjacent >= max_adjacent)) {
    return true;
  } else {
    return false;
  }
}

int changeSeats(std::vector<std::string>& grid,
                                 const int& part) {
  bool changed = true;
  int occupied = 0;
  int rows = grid.size();
  int columns = grid[0].size();
  // need to copy grid so all positions can be checked simultaneously

  while (changed) {
    // iterate over the grid and change the seating if necessary
    changed = false;
    occupied = 0;
    std::vector<std::string> tmp = grid;
    for (int i = 0; i < rows; i++) {
      for (int j = 0; j < columns; j++) {
        // easier access
        const char gridpoint = tmp[i][j];

        // check if we should change the grid
        if (gridpoint == '.') {
          continue;
        } else if (gridpoint == 'L') {
          if (changeSeat(tmp, i, j, part)) {
            grid[i][j] = '#';
            changed = true;
          }
        } else if (gridpoint == '#') {
          if (changeSeat(tmp, i, j, part)) {
            grid[i][j] = 'L';
            changed = true;
          }
        } else {
          std::cerr << "Invalid gridpoint state. ABORT.\n";
          exit(1);
        }
        // count number of occupied seats
        if (grid[i][j] == '#') occupied++;
      } // j
    } // i
  } // changed
  return occupied;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> grid;
  readInput(std::string(argv[1]), grid);
  std::vector<std::string> grid2 = grid; // start clean for part 2

  // part 1
  int occupied = changeSeats(grid, 1);
  std::cout << "Answer Part 1: " << occupied << "\n";

  // part 2
  occupied = changeSeats(grid2, 2);
  std::cout << "Answer Part 2: " << occupied << "\n";

 
} 
