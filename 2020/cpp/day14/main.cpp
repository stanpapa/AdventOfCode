#include "../utils.h"

#include <regex>
#include <map>
#include <bitset>
#include <sstream>
#include <set>
#include <math.h>

const std::regex rgx(R"([0-9]+))");

unsigned long int initializeProgram2(const std::vector<std::string> program) {
  std::string mask;
  std::map<unsigned long int, unsigned long int> mem;

  for (const std::string& s : program) {
    std::vector<std::string> tmp = split(s);
    if (tmp[0] == "mask") {
      mask = tmp[2];
      continue;
    }

    // find mem location
    std::smatch m;
    while(std::regex_search(tmp[0],m,rgx)) {
      break;
    }

    // convert memory location to 36-bit
    std::string memInBits = std::bitset<36>(std::stoi(m.str())).to_string();

    // apply mask
    int countX = 0;
    for (int i = 0; i < mask.size(); i++) {
      if (mask[i] == '0') {
        continue;
      } else if (mask[i] == '1') {
        memInBits[i] = '1';
      } else if (mask[i] == 'X') {
        memInBits[i] = 'X';
        countX++;
      } else {
        std::cerr << "Unrecognized command in bitmask. ABORT.\n";
        exit(1);
      }
    } // i

    // we have 2^8 possible permutations of the memory address

    // first generate countX + 1 combinations
    std::vector<std::string> allCombs; allCombs.reserve(countX+1);
    for (int i = 0; i <= countX; i++) {
      std::string tmp;
      tmp.reserve(countX);
      for (int j = 0; j < countX-i; j++) {
        tmp.append("0");
      }
      for (int j = 0; j < i; j++) {
        tmp.append("1");
      }
      allCombs.push_back(tmp);
    }

    // generate all 2^n permutations
    std::vector<std::string> allPerms; allPerms.reserve(pow(2,countX));
    for (auto& comb : allCombs) {
      do {
        allPerms.push_back(comb);
      } while (std::next_permutation(comb.begin(), comb.end()));
    }

    // use set so we remove duplicates
    std::set<std::string> addresses;
    for (const std::string& perm : allPerms) {
      int count = 0;
      std::string newMem = memInBits;
      for (int i = 0; i < memInBits.size(); i++) {
        if (memInBits[i] == 'X') {
          newMem[i] = perm[count];
          count++;
        }
      }
      addresses.insert(newMem);
    }

    
    // put value into all addresses
    for (const auto& address : addresses) {
      // convert 36-bit back to int
      std::bitset<36> bit;
      std::istringstream is(address);
      is >> bit;

      // store value in mem
      mem[bit.to_ulong()] = std::stoul(tmp[2]);
    }

  } // s

  // return sum of all values in memory
  unsigned long int sum = 0;
  for (const auto& [m, value] : mem) sum += value;
  return sum;
}

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
  std::cout << "Answer Part 2: " << initializeProgram2(program) << "\n";
  
  return 0;
} 
