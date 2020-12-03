#include "../utils.h"

size_t TreesEncountered(const std::vector<std::string>& input, const int& slopeX, const int& slopeY) {
  size_t N = 0;
  int x = 0;
  int gridWidth = input[0].size();
  int gridLength = input.size();

  for (int y = 0; y < gridLength; y+=slopeY) {
    if (input[y][x] == '#') N++;
    x += slopeX;
    if (x >= gridWidth) x -= gridWidth; // period boundary condition
  }
  return N;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);

  size_t nTrees = TreesEncountered(input, 3, 1);
  std::cout << "Answer Part 1: " << nTrees << "\n";
  

  nTrees *= TreesEncountered(input, 1, 1);
  nTrees *= TreesEncountered(input, 5, 1);
  nTrees *= TreesEncountered(input, 7, 1);
  nTrees *= TreesEncountered(input, 1, 2);
  std::cout << "Answer Part 2: " << nTrees << "\n";

  return 0;
} 
