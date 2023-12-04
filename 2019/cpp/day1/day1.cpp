#include <iostream>
#include <fstream>
using namespace std;

int main () {
    
    string line;

    ifstream file ("day1.inp");

    if (file.is_open()) {
        int fuel = 0;

        // part 1
        fuel = 0;
        while ( getline (file,line) ) {
          fuel += stoi(line) / 3 - 2;

        }

        // printf("Total fuel requirements: %i \n", sum);
        cout << "Total fuel requirements (1): " << fuel << "\n";

        // return to beginning of file
        file.clear();
        file.seekg(0, ios::beg);

        // part 2
        fuel = 0;
        int fuel_for_fuel = 0;
        while ( getline (file,line) ) {
          fuel += stoi(line) / 3 - 2;
          fuel_for_fuel = stoi(line) / 3 - 2;

          while (fuel_for_fuel / 3 - 2 > 0) {
            fuel += fuel_for_fuel / 3 - 2;
            fuel_for_fuel = fuel_for_fuel / 3 - 2;
          }
        }
        cout << "Total fuel requirements (2): " << fuel << "\n";

        file.close();
      }

    return 0;
}