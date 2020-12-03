#include "utils.h"

void readInput(const std::string& fname, std::vector<int>& input) {
  std::ifstream f(fname);
  std::string line;
  while (getline(f, line)) {
    input.push_back(std::stoi(line));
  }
  f.close();
}

void readInput(const std::string& fname, std::vector<std::string>& input) {
  std::ifstream f(fname);
  std::string line;
  while (getline(f, line)) {
    input.push_back(line);
  }
  f.close();
}

void print(const std::vector<std::string>& vec) {
  for (const std::string& s : vec) {
    std::cout << s << "\n";
  }
}
