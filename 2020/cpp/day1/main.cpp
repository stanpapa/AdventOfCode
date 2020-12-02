#include "../utils.h"

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input from file
  std::vector<int> entries;
  readInput(std::string(argv[1]), entries);


  // part 1
  for (int i = 0; i < entries.size(); i++) {
    for (int j = i + 1; j < entries.size(); j++) {
      int sum = entries[i] + entries[j];
      if (sum == 2020) {
        int product = entries[i] * entries[j];
        std::cout << "Answer Part 1: " << product << "\n";
      }
    }
  }

  // part 2
  for (int i = 0; i < entries.size(); i++) {
    for (int j = i + 1; j < entries.size(); j++) {
      int sum_ij = entries[i] + entries[j];
      if (sum_ij >= 2020) continue;
      for (int k = j + 1; k < entries.size(); k++) {
        int sum = sum_ij + entries[k];
        if (sum == 2020) {
          int product = entries[i] * entries[j] * entries[k];
          std::cout << "Answer Part 2: " << product << "\n";
        }
      } // k
    } // j
  } // i

} 
