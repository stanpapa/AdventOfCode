#include "../utils.h"

#include <regex>
#include <map>
#include <math.h>

#define PI 3.14159265

const static std::regex rgx("[NSEWLRF]|[0-9]+");

/**
 *  Rotate using rotation matrix. This breaks down if we do not rotate with
 *  a multiple of 90 degrees (because of int)
 */
void rotateVector(std::pair<int, int>& vector, const int& value, const int& factor) {
  int x = vector.first;
  int y = vector.second;
  vector.first  = round(         x * cos(value*PI/180) - factor * y * sin(value*PI/180));
  vector.second = round(factor * x * sin(value*PI/180) +          y * cos(value*PI/180));
}

int navigate(const std::vector<std::pair<char,int>>& instructions,
             std::pair<int, int>& waypoint, const int& part) {
  std::pair<int, int> coords = {0, 0}; // starting coordinates
  const std::map<char, std::pair<int, int>> shift_cardinal = {
    {'N', {0,1}},
    {'S', {0,-1}},
    {'E', {1,0}},
    {'W', {-1,0}}
  };

  // loop over all instructions
  for (const auto& [action, value] : instructions) {
    if (action == 'R') {
      rotateVector(waypoint, value, -1);
    } else if (action == 'L') {
      rotateVector(waypoint, value, 1);
    } else if (action == 'F' ) {
      coords.first += value * waypoint.first;
      coords.second += value * waypoint.second;
    } else {
      if (part == 1) {
        coords.first += value * shift_cardinal.at(action).first;
        coords.second += value * shift_cardinal.at(action).second;
      } else {
        waypoint.first += value * shift_cardinal.at(action).first;
        waypoint.second += value * shift_cardinal.at(action).second;
      }
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
  // split string into action and value and store as vector of pairs
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
  std::pair<int, int> waypoint = {1, 0};
  int manhattan = navigate(instructions, waypoint, 1);
  std::cout << "Answer Part 1: " << manhattan << "\n";

  // part 2
  waypoint = {10,1};
  manhattan = navigate(instructions, waypoint, 2);
  std::cout << "Answer Part 2: " << manhattan << "\n";
} 
