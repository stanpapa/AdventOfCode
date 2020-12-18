#include "../utils.h"

#include<unordered_set>
#include <tuple>
#include <chrono>

const std::vector<std::tuple<int, int, int>> directions = {
        {1,1, 0}, {1,0, 0}, {1,-1, 0}, {0,1, 0}, {0,-1, 0}, {-1,1, 0}, {-1,0, 0}, {-1,-1, 0},
        {1,1, 1}, {1,0, 1}, {1,-1, 1}, {0,1, 1}, {0,-1, 1}, {-1,1, 1}, {-1,0, 1}, {-1,-1, 1}, {0,0, 1},
        {1,1,-1}, {1,0,-1}, {1,-1,-1}, {0,1,-1}, {0,-1,-1}, {-1,1,-1}, {-1,0,-1}, {-1,-1,-1}, {0,0,-1}
};

struct Coord3D {
    int x_[3] = {0};

    Coord3D() = default;

    // copy constructor
    Coord3D(const Coord3D &other) {
      std::copy(std::begin(other.x_), std::end(other.x_), std::begin(x_));
    };

    Coord3D(const int &x, const int &y, const int &z) {
      x_[0] = x;
      x_[1] = y;
      x_[2] = z;
    };

    bool operator==(const Coord3D &other) const {
      return (x_[0] == other.x_[0]) &&
             (x_[1] == other.x_[1]) &&
             (x_[2] == other.x_[2]);
    };

    friend std::ostream &operator<<(std::ostream &os, const Coord3D &c) {
      return os << "(" << c.x_[0] << "," << c.x_[1] << "," << c.x_[2] << ")";
    }
};

template <>
// hopefully this is an OK hash function
struct std::hash<Coord3D> {
    size_t operator()( const Coord3D& toHash) const {
        size_t hash = 0;
        for (const int& i : toHash.x_) hash += std::hash<int>{}(i);
        return hash;
    };
};

// template <class T>
// struct Grid {
struct Grid3D {
  // std::unordered_set<T> xyz;
  std::unordered_set<Coord3D, std::hash<Coord3D>> active_;
  std::unordered_set<Coord3D, std::hash<Coord3D>> inactive_;

  // Grid();
  Grid3D() = default;
  Grid3D(const Grid3D& other) {
    active_ = other.active_;
    inactive_ = other.inactive_;
  }; // copy constructor

  int nActive() const { return active_.size(); }
  int nInactive() const { return inactive_.size(); }
  void clear() { active_.clear(); inactive_.clear(); }
  int size() const { return active_.size() + inactive_.size(); }

  std::tuple<int, int, int, int, int, int> xyzBounds() const {
    int minX = 0, maxX = 0;
    int minY = 0, maxY = 0;
    int minZ = 0, maxZ = 0;
    for (const auto& c : active_) {
      if (c.x_[0] < minX) minX = c.x_[0];
      if (c.x_[0] > maxX) maxX = c.x_[0];
      if (c.x_[1] < minY) minY = c.x_[1];
      if (c.x_[1] > maxY) maxY = c.x_[1];
      if (c.x_[2] < minZ) minZ = c.x_[2];
      if (c.x_[2] > maxZ) maxZ = c.x_[2];
    }
    for (const auto& c : inactive_) {
      if (c.x_[0] < minX) minX = c.x_[0];
      if (c.x_[0] > maxX) maxX = c.x_[0];
      if (c.x_[1] < minY) minY = c.x_[1];
      if (c.x_[1] > maxY) maxY = c.x_[1];
      if (c.x_[2] < minZ) minZ = c.x_[2];
      if (c.x_[2] > maxZ) maxZ = c.x_[2];
    }
    return std::make_tuple(minX, maxX, minY, maxY, minZ, maxZ);
  }

  // call operator
  Grid3D& operator()(const Grid3D& other) {
    active_ = other.active_;
    inactive_ = other.inactive_;
    return *this;
  };
  friend std::ostream& operator<<(std::ostream& os, const Grid3D& grid) {
    std::cout << "active:\n";
    for (const auto & c : grid.active_) {
      os << "(" << c.x_[0] << "," << c.x_[1] << "," << c.x_[2] << ")\n";
    }
    std::cout << "inactive:\n";
    for (const auto & c : grid.inactive_) {
      os << "(" << c.x_[0] << "," << c.x_[1] << "," << c.x_[2] << ")\n";
    }
    return os;
  }
};

// typedef Grid<Coord3D> Grid3D;

void gameOfLife(Grid3D& grid3d) {

  // RULES PART 1
  // 1. If a cube is active and exactly 2 or 3 of its neighbors are also active,
  //    the cube remains active. Otherwise, the cube becomes inactive.
  // 2. If a cube is inactive but exactly 3 of its neighbors are active,
  //    the cube becomes active. Otherwise, the cube remains inactive.

  for (int cycle = 0; cycle < 6; cycle++) {
    // add inactive planes on the outside of the grid
    // needed for expanding into 3D
    {
      auto [minX, maxX, minY, maxY, minZ, maxZ] = grid3d.xyzBounds();
      for (int z = minZ-1; z <= maxZ+1; z++) {
        for (int x = minX-1; x <= maxX+1; x++) {
          for (int y = minY-1; y <= maxY+1; y++) {
            if (z >= minZ && z <= maxZ) {
              if (x >= minX && x <= maxX && y >= minY && y <= maxY) continue;
              grid3d.inactive_.insert(Coord3D(x, y, z));
            } else {
              grid3d.inactive_.insert(Coord3D(x, y, z));
            }
          } // y
        } // x
      } // z
    }
    Grid3D tmp(grid3d);
    grid3d.clear();

    // play
    for (const auto& coord : tmp.active_) {
      int count = 0;
      for (const auto& [dx, dy, dz] : directions) {
        Coord3D adjacent(coord.x_[0]+dx,coord.x_[1]+dy,coord.x_[2]+dz);
        if (tmp.active_.find(adjacent) != tmp.active_.end()) {
          count++;
        }
      } // directions

      if (count == 2 || count == 3) {
        grid3d.active_.insert(coord);
      } else {
        grid3d.inactive_.insert(coord);
      }
    } // coords active

    for (const auto& coord : tmp.inactive_) {
      int count = 0;
      for (const auto& [dx, dy, dz] : directions) {
        Coord3D adjacent(coord.x_[0]+dx,coord.x_[1]+dy,coord.x_[2]+dz);
        if (tmp.active_.find(adjacent) != tmp.active_.end()) count++;
      } // directions
      if (count == 3) {
        grid3d.active_.insert(coord);
      } else {
        grid3d.inactive_.insert(coord);
      }
    } // coords inactive
  } // cycle
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input
  std::vector<std::string> input;
  readInput(std::string(argv[1]), input);

  // part 1
  Grid3D grid3d;
  for (int i = 0; i < input.size(); i++) {
    for (int j = 0; j < input[0].size(); j++) {
      if (input[i][j] == '#') {
        grid3d.active_.insert(Coord3D(i,j,0));
      } else {
        grid3d.inactive_.insert(Coord3D(i,j,0));
      }
    }
  }
  auto started = std::chrono::high_resolution_clock::now();
  gameOfLife(grid3d);
  std::cout << "Answer Part 1: " << grid3d.nActive() << "\n";
  auto done = std::chrono::high_resolution_clock::now();
  std::cout << "Time elapsed: " << std::chrono::duration_cast<std::chrono::milliseconds>(done-started).count() << "ms\n";

  // part 2

 
} 
