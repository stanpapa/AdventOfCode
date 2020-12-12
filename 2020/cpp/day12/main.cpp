#include "../utils.h"

#include <regex>
#include <map>
#include <math.h>

#define PI 3.14159265

const static std::regex rgx("[NSEWLRF]|[0-9]+");

/**
 *  Rotate using linear algebra. This breaks down if we do not rotate with
 *  a multiple of 90 degrees (because of int)
 */
void rotateShip(std::pair<int, int>& direction, const int& value, const int& factor) {
  int x_dir_old = direction.first;
  int y_dir_old = direction.second;
  direction.first = x_dir_old * cos(value*PI/180) - factor * y_dir_old * sin(value*PI/180);
  direction.second = factor * x_dir_old * sin(value*PI/180) + y_dir_old * cos(value*PI/180);
}

int navigate(const std::vector<std::pair<char,int>>& instructions) {
  std::pair<int, int> direction = {1, 0}; // we start facing east
  std::pair<int, int> coords = {0, 0}; // starting coordinates
  std::map<char, std::pair<int, int>> shift_cardinal = {
    {'N', {0,1}},
    {'S', {0,-1}},
    {'E', {1,0}},
    {'W', {-1,0}}
  };

  for (const auto& [action, value] : instructions) {
    if (action == 'R') {
      rotateShip(direction, value, -1);
    } else if (action == 'L') {
      rotateShip(direction, value, 1);
    } else if (action == 'F' ) {
      coords.first += value * direction.first;
      coords.second += value * direction.second;
    } else {
      coords.first += value * shift_cardinal.at(action).first;
      coords.second += value * shift_cardinal.at(action).second;
    }
  }

  // return manhattan distance
  return abs(coords.first) + abs(coords.second);
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);
  std::vector<std::pair<char, int>> instructions;
  instructions.reserve(input.size());

  // practice regex
  for (const std::string& s : input) {
    std::vector<std::string> tmp;
    std::sregex_token_iterator iter(s.begin(), s.end(), rgx);
    std::sregex_token_iterator end;
    while (iter != end) {
      tmp.push_back(*iter);
      iter++;
    }
    instructions.emplace_back(tmp[0][0], std::stoi(tmp[1]));
  }

  // part 1
  int manhattan = navigate(instructions);
  std::cout << "Answer Part 1: " << manhattan << "\n";

  // part 2

 
} 
