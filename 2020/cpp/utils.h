#ifndef UTILS_H
#define UTILS_H

#include <iostream>
#include <fstream>

#include <string>
#include <vector>

void readInput(const std::string& fname, std::vector<int>& input);
void readInput(const std::string& fname, std::vector<std::string>& input);

void print(const std::vector<std::string>& vec);

#endif
