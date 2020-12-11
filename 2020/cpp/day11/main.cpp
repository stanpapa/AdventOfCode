#include "../utils.h"

bool changeSeat(const std::vector<std::vector<char>>& grid,
                     const int& i, const int& j) {
  int adjacent = 0;
  int rows = grid.size();
  int columns = grid[0].size();
  
  // count number of adjacent occupied seats
  for (int x = i - 1; x <= i + 1; x++) {
    for (int y = j - 1; y <= j + 1; y++) {
      if (x < 0 || x >= rows) continue;
      if (y < 0 || y >= columns) continue;
      if (x == i && y == j) continue;
      if (grid[x][y] == '#') adjacent++;
    }
  }

  // depending on the rules we have to change the seat
  if ((grid[i][j] == 'L' && adjacent == 0) || 
      (grid[i][j] == '#' && adjacent >= 4)) {
    return true;
  } else {
    return false;
  }
}

std::pair<bool, int> changeSeats(std::vector<std::vector<char>>& grid) {
  bool changed = false;
  int occupied = 0;
  int rows = grid.size();
  int columns = grid[0].size();
  std::vector<std::vector<char>> tmp = grid;

  // RULES:
  // 1. If a seat is empty (L) and there are no occupied seats adjacent to it,
  //    the seat becomes occupied.
  // 2. If a seat is occupied (#) and four or more seats adjacent to it
  //    are also occupied, the seat becomes empty.
  // 3. Otherwise, the seat's state does not change.

  for (int i = 0; i < rows; i++) {
    for (int j = 0; j < columns; j++) {
      // easier access
      const char gridpoint = tmp[i][j];

      // check if we should change the grid
      if (gridpoint == '.') {
        continue;
      } else if (gridpoint == 'L') {
        if (changeSeat(tmp, i, j)) {
          grid[i][j] = '#';
          changed = true;
        }
      } else if (gridpoint == '#') {
        if (changeSeat(tmp, i, j)) {
          grid[i][j] = 'L';
          changed = true;
        }
      } else {
        std::cerr << "Invalid gridpoint state. ABORT.\n";
        exit(1);
      }
      // count number of occupied seats
      if (grid[i][j] == '#') occupied++;
    }
  }
  return std::make_pair(changed, occupied);
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::vector<char>> grid;
  readInputGrid(std::string(argv[1]), grid);

  // part 1
  bool seatsChanged = true;
  int occupied;
  while (seatsChanged) {
    std::tie(seatsChanged, occupied) = changeSeats(grid);
  }
  std::cout << "Answer Part 1: " << occupied << "\n";

  // part 2
  std::cout << "Answer Part 2: " << "\n";

 
} 
