#include <iostream>
#include <fstream>
#include <sstream>
#include <vector>
#include <string>
using namespace std;

vector<int> read_ints()
{

	vector<int> result;
    ifstream input ("day2.inp");

    if (input.is_open()) {

	    string numbers_string;
	    getline(input,numbers_string);
	    stringstream ss(numbers_string);

		while( ss.good() ) {
		    string substr;
		    getline( ss, substr, ',' );
		    result.push_back( stoi(substr) );
		}

	    input.close();
    }

	return result;
}

int parameter0 (vector<int> numbers, int noun, int verb) {

	numbers[1] = noun;
   	numbers[2] = verb;

   	for (int i = 0; i < numbers.size();) {
   		if (numbers[i] == 1) {
   			numbers[numbers[i+3]] = numbers[numbers[i+1]] + numbers[numbers[i+2]];
   		}
   		else if (numbers[i] == 2) {
   			numbers[numbers[i+3]] = numbers[numbers[i+1]] * numbers[numbers[i+2]];
   		}
   		else if (numbers[i] == 99) {
   			break;
   		}
   		else {
   			cout << "Something went horribly wrong. ABORT\n";
   			exit(1);
   		}
   		i += 4;
   	}
   	return numbers[0];
}

int main () {

   	vector<int> numbers = read_ints();

   	cout << "1: " << parameter0(numbers,12,2) << "\n";
   	
   	// reset vector
	numbers = read_ints();

	// find noun and verb by brute force
	int noun, verb;
	for (int i = 0; i <= 99; i++) {
		for (int j = 0; j <= 99; j++) {
			if (parameter0(numbers,i,j) == 19690720) {
				noun = i;
				verb = j;
				goto end;
			}
		}
	}

	end:
	cout << "2: " << 100*noun+verb << "\n";

    return 0;
}