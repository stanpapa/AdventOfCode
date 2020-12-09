#include "../utils.h"

int main(int argc, char* argv[]) {

  int preamble = 25;
  if (argc < 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }
  if (argc == 3) preamble = std::stoi(argv[2]);

  // read input
  // unsigned long long is needed to represent all numbers in the input
  std::vector<unsigned long long int> input;
  readInput(std::string(argv[1]), input);

  // part 1
  // find the number that is not the sum of two numbers in its preamble
  unsigned long long num;
  for (int i = preamble; i < input.size(); i++) {
    num = input[i];
    bool foundSum = false;
    // only sum over unique pairs in the preamble
    for (int j = i - preamble; j < i; j++) {
      for (int k = i - preamble; k < j; k++) {
        if (num == input[j] + input[k]) {
          foundSum = true;
          break;
        }
      } // k
      if (foundSum) break;
    } // j
    // found the outlier!
    if (!foundSum) break;
  } // i
  std::cout << "Answer Part 1: " << num << "\n";

  // part 2
  // find a set of at least two contiguous numbers which sum to the invalid number of part 1
  int min = 0, max = 1;
  bool weaknessFound = false;
  while (!weaknessFound) {
    // calculate sum within the range
    auto sum = 0;
    for (int i = min; i <= max; i++) {
      sum += input[i];
    }

    // found the contiguous numbers!
    if (sum == num) {
      weaknessFound = true;
      // answer to part 2 is the sum of the smallest and largest value in the range
      auto smallest = input[min], largest = input[min];
      for (int i = min; i <= max; i++) {
        if (smallest > input[i]) smallest = input[i];
        if (largest  < input[i]) largest  = input[i];
      }
      num = smallest + largest;
    } else if (sum < num) {
      max++;
      // numbers must be contiguous, so they cannot contain the value we're looking for
      if (input[max] == num) {
        min = max + 1;
        max = min + 1;
      }
    } else { // sum > num
      min++;
      // numbers must be contiguous, so they cannot contain the value we're looking for
      if (input[min] == num) {
        min++;
      }
      max = min + 1;
    }
  } // while
  std::cout << "Answer Part 2: " << num << "\n";

  return 0;

} 
