#include "utils.h"

#include <sstream>

std::vector<std::string> split(const std::string& input, const char* delimiter) {
  std::vector<std::string> ret;

  char *p, *c_str;

  c_str = new char[input.size() + 1];
  std::strncpy(c_str, input.c_str(), input.size() + 1);

  p = std::strtok(c_str, delimiter);
  while (p != nullptr) {
    ret.emplace_back(p);
    p = std::strtok(nullptr, delimiter);
  }

  delete[] c_str;

  return ret;
}

void readInput(const std::string& fname, std::vector<int>& input) {
  std::ifstream f(fname);
  std::string line;
  while (getline(f, line)) {
    input.push_back(std::stoi(line));
  }
  f.close();
}

void readInput(const std::string& fname, std::vector<unsigned long long int>& input) {
  std::ifstream f(fname);
  std::string line;
  while (getline(f, line)) {
    input.push_back(std::stoull(line));
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

void readInput(const std::string& fname, std::string& input) {
  std::ifstream f(fname);
  std::stringstream ss;
  ss << f.rdbuf();
  input = ss.str();
}

void readInputAndSplit(const std::string& fname, std::vector<std::string>& input) {
  std::ifstream f(fname);
  std::string line, token;

  while (getline(f, line)) {
    if (!line.empty()) {
      token.append(line + " ");
    } else {
      input.push_back(token);
      token = "";
    }
  }
  input.push_back(token); // final entry
  f.close();
}

bool isNumber(const std::string& s) {
  return !s.empty() && std::find_if(s.begin(),s.end(),
          [](unsigned char c) { return !std::isdigit(c); }) == s.end();
}
