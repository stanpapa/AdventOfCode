#include <iostream>
#include <fstream>

#include <vector>

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input from file
  std::string fname = std::string(argv[1]);
  std::vector<int> entries;
  std::ifstream f(fname);
  std::string line;
  while (getline(f, line)) {
    entries.push_back(std::stoi(line));
  }
  f.close();


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
