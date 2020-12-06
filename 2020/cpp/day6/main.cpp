#include "../utils.h"

#include <set>
#include <map>

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input and create raw passports
  std::vector<std::string> input;
  readInputAndSplit(std::string(argv[1]), input);


  // part 1
  int sumTotal = 0;
  for (const std::string& group : input) {
    // create unique set of questions that have been answered with yes
    std::set<char> yesQuestions;
    for (const char& c : group) {
      if (isspace(c)) continue;
      yesQuestions.insert(c);
    }
    // add questions answered with yes to sum
    sumTotal += yesQuestions.size();
  }
  std::cout << "Answer Part 1: " << sumTotal << "\n";

  // part 2
  sumTotal = 0;
  for (const std::string& group : input) {
    // because of trailing whitespace this needs to be 0
    int nPersonsPerGroup = 0;
    std::map<char, int> yesCount;
    // count the number of yesses per group
    for (const char& c : group) {
      if (isspace(c)) {
        nPersonsPerGroup++;
        continue;
      }
      yesCount[c]++;
    }
    // check which questions have all been answered with yes in the group
    for (const auto& question : yesCount) {
      if (question.second == nPersonsPerGroup) sumTotal++;
    }
  }
  std::cout << "Answer Part 2: " << sumTotal << "\n";

  return 0;
} 
