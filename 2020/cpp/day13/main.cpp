#include "../utils.h"

#include <regex>

const static std::regex rgx(R"([0-9]+|x)");

int streamlineDepartures(const std::vector<int>& busIDs) {
  
}

int waitForBus(const int& earliestTimestamp, const std::vector<int> busIDs) {
  int minimalWait = 10000;
  int earliestBus = -1;

  for (const int& bus : busIDs) {
    if (bus == -1) continue; // skip x
    int n = earliestTimestamp / bus + 1;
    int diff = n * bus - earliestTimestamp;
    if (diff < minimalWait) {
      minimalWait = diff;
      earliestBus = bus;
    }
  }

  return minimalWait * earliestBus;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  int earliestTimestamp = -1;
  std::vector<int> busIDs;
  {
    std::ifstream f(argv[1]);
    std::string line;
    getline(f, line);
    earliestTimestamp = std::stoi(line);
    getline(f, line);

    // practice regex
    std::sregex_token_iterator iter(line.begin(), line.end(), rgx);
    std::sregex_token_iterator end;
    while (iter != end) {
      int tmp;
      if (*iter == 'x') {
        tmp = -1;
      } else {
        tmp = std::stoi(*iter);
      }
      busIDs.push_back(tmp);
      iter++;
    }
  }

  // part 1
  std::cout << "Answer Part 1: " << waitForBus(earliestTimestamp, busIDs) << "\n";

  // part 2
  std::cout << "Answer Part 2: " << streamlineDepartures(busIDs) << "\n";
  
  return 0;
} 
