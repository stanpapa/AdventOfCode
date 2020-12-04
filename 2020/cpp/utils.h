#ifndef UTILS_H
#define UTILS_H

#include <iostream>
#include <fstream>

#include <string>
#include <vector>

std::vector<std::string> split(const std::string& input, const char* delimeter = " ");

void readInput(const std::string& fname, std::vector<int>& input);
void readInput(const std::string& fname, std::vector<std::string>& input);
void readInput(const std::string& fname, std::string& input);
void readInputAndSplit(const std::string& fname, std::vector<std::string>& input);

void print(const std::vector<std::string>& vec);

bool isNumber(const std::string& s);

#endif
