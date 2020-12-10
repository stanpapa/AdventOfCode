#include "../utils.h"

#include <map>

long long int permutations(const int& n) {
  if (n <= 1) return 1;

  if (n == 2) {
    return 2;
  } else if (n == 3) {
    return 4;
  } else if (n == 4) {
    return 7;
  } else {
    return 0;
  }
}

int main(int argc, char* argv[]) {

  int preamble = 25;
  if (argc < 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<int> input = {0}; // charging outlet has 0 jolts
  readInput(std::string(argv[1]), input);
  std::sort(input.begin(), input.end());
  input.push_back(input.back()+3); // device's in-built adapter is rated 3 jolts higher than the highest-rated adapter

  // part 1
  // count the jolt differences between connecting adapters
  // multiple the number of 1-jolt differences by the number of 3-jolt differences for the answer
  std::map<int, int> diffMap = {{1,0}, {2,0}, {3,0}};
  for (int i = 1; i < input.size(); i++) {
    int diff = input[i] - input[i-1];
    diffMap[diff]++;
  }
  std::cout << "Answer Part 1: " << diffMap.at(1) * diffMap.at(3) << "\n";

  // part 2
  // only thing that can change is 1-jolt jumps
  long long int arrangements = 1;
  int count = 0;
  for (int i = 1; i < input.size(); i++) {
    int diff = input[i] - input[i-1];
    if (diff == 1) {
      count++;
    } else {
      arrangements *= permutations(count);
      count = 0;
    }
  }
  std::cout << "Answer Part 2: " << arrangements << "\n";

  return 0;

} 
