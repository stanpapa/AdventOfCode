#include <iostream>
#include <fstream>

#include <vector>

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  std::vector<std::string> entries;
  // read input from file
  {
    std::string fname = std::string(argv[1]);
    std::ifstream f(fname);
    std::string line;
    while (getline(f, line)) {
      entries.push_back(line);
    }
    f.close();
  }

  int validPwds = 0;
  // part 1
  for (const std::string& entry : entries) {
    size_t pos = entry.find("-");
    int min = std::stoi(entry.substr(0,pos));
    int max = std::stoi(entry.substr(pos+1,entry.find(" ")));
    pos = entry.find(":");
    char letter = entry.substr(pos-1,1)[0];
    std::string pwd = entry.substr(pos+2, entry.size()-pos-2);

    int count = 0;
    for (const char& c : pwd) {
      if (c == letter) count++;
    }
    if (count >= min && count <= max) validPwds++;
  } // entry

  std::cout << "Number of valid passwords: " << validPwds << "\n";
 
} 
