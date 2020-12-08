#include "../utils.h"

#include <set>

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);

  // part 1
  std::set<int> executedInstructions;
  int acc = 0;
  for (int i = 0; i < input.size(); i++) {
    // break out of (possible) infinite loop
    if (executedInstructions.find(i) != executedInstructions.end()) break;

    // add instruction number to seen instructions
    executedInstructions.insert(i);
    std::vector<std::string> tmp = split(input[i]);
    // execute command
    if (tmp[0] == "nop") {
      continue;
    } else if (tmp[0] == "acc") {
      acc += std::stoi(tmp[1]);
    } else if (tmp[0] == "jmp") {
      i += std::stoi(tmp[1]) - 1; // -1 since the for loop also does i++
    } else {
      std::cerr << "Instruction not recognized! ABORT.\n";
    }
  }
  std::cout << "Answer Part 1: " << acc << "\n";

  // part 2
  // brute-forcing it. is there a cleverer way?
  for (int i = 0; i < input.size(); i++) {
    // skip acc instruction, only interested in jmp and nop
    if (input[i].find("acc") != std::string::npos) continue;

    executedInstructions.clear();
    acc = 0;
    bool foundCorrectInstructions = false;
    for (int j = 0; j < input.size(); j++) {
      if (j == input.size() - 1) foundCorrectInstructions = true;
      // break out of (possible) infinite loop
      if (executedInstructions.find(j) != executedInstructions.end()) break;
  
      executedInstructions.insert(j);
      std::vector<std::string> tmp = split(input[j]);
      // swap a single instruction
      if (i == j) {
        if (tmp[0] == "nop") {
          tmp[0] = "jmp";
        } else if (tmp[0] == "jmp") {
          tmp[0] = "nop";
        }
      }
      if (tmp[0] == "nop") {
        continue;
      } else if (tmp[0] == "acc") {
        acc += std::stoi(tmp[1]);
      } else if (tmp[0] == "jmp") {
        j += std::stoi(tmp[1]) - 1; // -1 since the for loop also does j++
      } else {
        std::cerr << "Instruction not recognized! ABORT.\n";
      }

    }
    if (foundCorrectInstructions) break;
  }
  std::cout << "Answer Part 2: " << acc << "\n";

  return 0;

} 
