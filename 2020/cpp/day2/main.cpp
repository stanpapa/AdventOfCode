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


} 
