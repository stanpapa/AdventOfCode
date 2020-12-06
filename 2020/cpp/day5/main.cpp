#include "../utils.h"

#include <map>

/**
 * @brief
 * Function to calculate seat ID from row and column
 */
int seatID(const int& row, const int& column) { return row * 8 + column; }

/**
 * @brief
 * Recursive algorithm to find the position of the seat according to the instructions on the boarding pass
 */
int determinePosition(std::string& s, int& min, int& max) {
  // if there are no instructions left, we have found the correct row/column
  if (s.empty()) return min;

  // instruction will always be the first character of the string
  const char instruction = s[0];
  s.erase(0,1);

  // find position recursively by setting a new min or max position
  int middle = (max - min) / 2;
  if (instruction == 'F' || instruction == 'L') {
    int newMax = min + middle;
    return determinePosition(s, min, newMax);
  } else if (instruction == 'B' || instruction == 'R') {
    int newMin = min + middle + 1;
    return determinePosition(s, newMin, max);
  } else {
    std::cout << "Unknown instruction. ABORT.\n";
    exit(1);
  }
}

/**
 * @brief
 * Determine the seat ID by calling the recursive position finder for both the row and the column
 */
int determineSeatID(std::string& s, int& row, int& column) {
  std::string seatInstructions = s.substr(0,7);
  int min = 0, max = 127;
  row =  determinePosition(seatInstructions, min, max);
  min = 0, max = 7;
  seatInstructions = s.substr(7,3);
  column = determinePosition(seatInstructions, min, max);
  return seatID(row, column);
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input and create raw passports
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);

  // part 1
  int maxSeatID = 0;
  std::map<int, std::vector<int>> takenSeats;
  for (const std::string& s : input) {
    std::string tmp = s; // go around the const-qualifier
    int row, column;
    int currentSeatID = determineSeatID(tmp, row, column);
    if (currentSeatID > maxSeatID) maxSeatID = currentSeatID;
    // make a map of all taken seats
    takenSeats[row].push_back(column);
  } // s
  std::cout << "Highest seat ID is " << maxSeatID << "\n";


  // part 2
  bool foundSeat = false;
  int mySeatID = -1;
  for (auto& seat : takenSeats) {
    // if a row is full, don't bother inspecting it for an empty seat
    if (seat.second.size() == 8) continue;
    int column = 0;
    // sort vector so we can systematically search for the empty seat by going from low to high
    std::sort(seat.second.begin(), seat.second.end());
    for (const auto& c : seat.second) {
      if (c != column) {
        mySeatID = seatID(seat.first, column);
        foundSeat = true;
        break;
      } 
      column++;
    }
    if (foundSeat) break;
  }
  if (mySeatID == -1) std::cerr << "I couldn't find my seat...\n";

  std::cout << "My seat ID is " << mySeatID << "\n";

  return 0;
} 
