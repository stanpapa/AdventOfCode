#include "../utils.h"

#include <regex>
#include <map>
#include <bitset>
#include <sstream>

const std::regex rgx(R"([0-9]+))");

unsigned long int initializeProgram(const std::vector<std::string> program) {
  std::string mask;
  std::map<int, unsigned long int> mem;

  for (const std::string& s : program) {
    std::vector<std::string> tmp = split(s);
    if (tmp[0] == "mask") {
      mask = tmp[2];
      continue;
    }

    // convert value to 36-bit
    std::string valueInBits = std::bitset<36>(std::stoi(tmp[2])).to_string();

    // apply mask
    for (int i = 0; i < mask.size(); i++) {
      if (mask[i] == 'X') {
        continue;
      } else if (mask[i] == '1') {
        valueInBits[i] = '1';
      } else if (mask[i] == '0') {
        valueInBits[i] = '0';
      } else {
        std::cerr << "Unrecognized command in bitmask. ABORT.\n";
        exit(1);
      }
    } // i

    // convert 36-bit back to int
    std::bitset<36> bit;
    std::istringstream is(valueInBits);
    is >> bit;

    // find mem location
    std::smatch m;
    while(std::regex_search(tmp[0],m,rgx)) {
      break;
    }
    // store value in mem
    mem[std::stoi(m.str())] = bit.to_ulong();
  } // s

  // return sum of all values in memory
  unsigned long int sum = 0;
  for (const auto& [m, value] : mem) sum += value;
  return sum;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> program;
  readInput(argv[1], program);
  
  // part 1
  std::cout << "Answer Part 1: " << initializeProgram(program) << "\n";

  // part 2
  std::cout << "Answer Part 2: " << "\n";
  
  return 0;
} 
