#include <iostream>
#include <fstream>
#include <sstream>
#include <string>
#include <vector>
#include <cmath>
#include <ctime>
using namespace std;

int Manhattan_distance(vector<int> v1, vector<int> v2) {

  // check if vectors span same space
  if (v1.size() != v2.size()) {
    cout << "Vectors do not span the same space. ABORT.\n";
    exit(1);
  }

  // compute taxicab distance
  int distance = 0;
  for (int i = 0; i < v1.size(); i++) {
    distance += abs(v1[i] - v2[i]);
  }

	return distance;
}

void get_wire(stringstream& ss, vector<vector<int>>& wire, vector<int> current_pos) {

  // store every coordinate of the wire, not just the corners
  while( ss.good() ) {
    string substr;
    getline( ss, substr, ',' );
    int step = stoi(substr.substr(1,substr.length()-1));
    for (int i = 0; i < step; i++) {
      if (substr[0] == 'R') {
        current_pos[0] = current_pos[0] + 1;
      }
      else if (substr[0] == 'L') {
        current_pos[0] = current_pos[0] - 1;
      }
      else if (substr[0] == 'U') {
        current_pos[1] = current_pos[1] + 1;
      }
      else if (substr[0] == 'D') {
        current_pos[1] = current_pos[1] - 1;
      }
      else {
        cout << "Not a valid direction. ABORT.\n";
        exit(2);
      }
      wire.push_back(current_pos);
    }    
  }
}

int get_closest_intersection_distance(vector<vector<int>>& wire1, vector<vector<int>>& wire2, vector<int> origin) {

  vector<vector<int>> intersections;

  // find the intersections
  // start at index 1, since wire[0] is the origin
  for (int i = 1; i < wire1.size(); i++) {
    for (int j = 1; j < wire2.size(); j++) {
      if (wire1[i][0] == wire2[j][0] && wire1[i][1] == wire2[j][1]) {
        intersections.push_back(wire1[i]);
      }
    }
  }

  // find the closest intersection
  int closest_intersection_distance = Manhattan_distance(intersections[1],origin);
  for (int i = 2; i < intersections.size(); i++) {
    if (Manhattan_distance(intersections[i],origin) < closest_intersection_distance) {
      closest_intersection_distance = Manhattan_distance(intersections[i],origin);
    }
  }

  return closest_intersection_distance;
}


int main () {

  ifstream input ("day3.inp");
  vector<int> origin{0,0};
  vector<vector<int>> wire1 {origin};
  vector<vector<int>> wire2 {origin};

  // save path of wire as vector of vectors
  if (input.is_open()) {

    string direction;

    getline(input,direction);
    stringstream ss(direction);
    get_wire(ss, wire1 ,origin);

    getline(input,direction);
    ss.clear();
    ss.str(direction);
    get_wire(ss, wire2, origin);
 
    input.close();
    
  }

  cout << "Manhattan distance: " << get_closest_intersection_distance(wire1, wire2, origin) << "\n";

	return 0;
}