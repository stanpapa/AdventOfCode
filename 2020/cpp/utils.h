#ifndef UTILS_H
#define UTILS_H

#include <iostream>
#include <fstream>

#include <string>
#include <vector>

std::vector<std::string> split(const std::string& input, const char* delimeter = " ");

void readInput(const std::string& fname, std::vector<int>& input);
void readInput(const std::string& fname, std::vector<unsigned long long int>& input);
void readInput(const std::string& fname, std::vector<std::string>& input);
void readInput(const std::string& fname, std::string& input);
void readInputAndSplit(const std::string& fname, std::vector<std::string>& input);

template <typename T>
void print(const std::vector<T>& vec) {
  for (const T& v : vec) std::cout << v << "\n";
}

bool isNumber(const std::string& s);

#endif
