#ifndef UTILS_H
#define UTILS_H

#include <iostream>
#include <fstream>

#include <string>
#include <vector>

template<typename T>
std::vector<T> readInput(const std::string& input) {
  std::vector<T> tmp;
  std::ifstream f(input);
  std::string line;
  while (getline(f, line)) {
    tmp.push_back(line);
  }
  f.close();

  return tmp;
}

#endif
