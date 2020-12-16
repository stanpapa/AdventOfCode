#include "../utils.h"

#include <regex>
#include <unordered_map>
#include <chrono>

const std::regex rgx(R"([0-9]+)");

int memoryGame(const std::vector<int>& input, const int& range) {
  std::unordered_map<int, std::vector<int>> numInTurns;

  // first play with the input
  // start counting at 1
  int lastNum = -1;
  for (int i = 1; i <= input.size(); i++){
    numInTurns[input[i-1]].push_back(i);
    lastNum = input[i-1];
  }

  for (int i = input.size()+1; i <= range; i++) {
    if (numInTurns[lastNum].size() == 1) {
      numInTurns[0].push_back(i);
      lastNum = 0;
    } else if (numInTurns[lastNum].size() > 1) {
      int n = numInTurns[lastNum].size();
      lastNum = numInTurns[lastNum][n-1] - numInTurns[lastNum][n-2];
      numInTurns[lastNum].push_back(i);
    } else {
      std::cerr << "Map is corrupted. ABORT.\n"; exit(1);
    }
  } // i
  return lastNum;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<int> input;
  {
    std::ifstream f(argv[1]);
    std::string line;
    getline(f, line);

    // practice regex
    std::sregex_token_iterator iter(line.begin(), line.end(), rgx);
    std::sregex_token_iterator end;
    while (iter != end) {
      input.push_back(std::stoi(*iter));
      iter++;
    }
    f.close();
  }

  // part 1
  auto started = std::chrono::high_resolution_clock::now();
  std::cout << "Answer Part 1: "  << memoryGame(input, 2020) << "\n";
  auto done = std::chrono::high_resolution_clock::now();
  std::cout << "Time elapsed: " << std::chrono::duration_cast<std::chrono::milliseconds>(done-started).count() << "ms\n";

  // part 2
  started = std::chrono::high_resolution_clock::now();
  std::cout << "Answer Part 2: "  << memoryGame(input, 30000000) << "\n";
  done = std::chrono::high_resolution_clock::now();
  std::cout << "Time elapsed: " << std::chrono::duration_cast<std::chrono::seconds>(done-started).count() << "s\n";
  
  return 0;
} 
