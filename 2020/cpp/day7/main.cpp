#include "../utils.h"

#include <map>
#include <regex>
#include <set>

const static std::regex rgx(R"((\sbags contain\s)|(\sbags?[.,]\s?))");
const static std::string target = "shiny gold";

typedef std::map<std::string, std::vector<std::pair<int, std::string>>> tree;

// ----------------------------------------
// Part 1
// ----------------------------------------
bool foundTarget(const tree& bagMap, const std::string& containing, std::set<std::string>& seen) {
  bool found = false;
  for (const auto& [num, bag] : bagMap.at(containing)) {
    if (num == 0) return false;
    if (bag == target) return true;
    if (seen.find(bag) != seen.end()) return true;
    found = foundTarget(bagMap, bag, seen);
    if (found) return true;
  }
  return false;
}

int countBagsContaining(const tree& bagMap) {
  std::set<std::string> seen;

  for (const auto& [containing, contained] : bagMap) {
      if (containing == target) continue;
      if (foundTarget(bagMap, containing, seen)) {
        seen.insert(containing);
      }
  } // [containing, contained]

  return seen.size();
}

// ----------------------------------------
// Part 2
// ----------------------------------------
int countBags(const tree& bagMap, const std::string& containing) {
  int count = 0;
  for (const auto& [num, bag] : bagMap.at(containing)) {
    if (num == 0) return 0;
    count += num;
    count += num * countBags(bagMap, bag);
  }
  return count;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);
  tree bagMap;

  // practice regex
  // create tree from input. example below
  // converts "light red bags contain 1 bright white bag, 2 muted yellow bags.""
  // to {light red: {{1, bright white}, {2, muted yellow}}}
  for (const std::string& s : input) {
    int count = 0;
    std::string container;
    std::vector<std::pair<int, std::string>> tmp;

    std::sregex_token_iterator iter(s.begin(), s.end(), rgx, -1);
    std::sregex_token_iterator end;
    while (iter != end) {
      if (count == 0) {
        // key of the map
        container = *iter;
      } else {
        // create vector of bags that are contained within the map key
        std::string s2 = *iter;
        if (s2 == "no other") {
          tmp.emplace_back(0, "");
        } else {
          // need to shift by 48, because of ASCII...
          tmp.emplace_back((int)s2[0]-48, s2.substr(2,s2.size()-2));
        }
      }
      iter++; count++;
    } // iter
    bagMap[container] = tmp;
  } // s

  // part 1
  std::cout << "Answer Part 1: " << countBagsContaining(bagMap) << "\n";

  // part 2
  std::string tmp = target;
  std::cout << "Answer Part 2: " << countBags(bagMap, tmp) << "\n";

  return 0;
} 
