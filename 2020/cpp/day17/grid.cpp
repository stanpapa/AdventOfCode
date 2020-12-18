#include "grid.h"
#include <vector>

const std::vector<std::tuple<int, int, int>> directions3D = {
        {1,1, 0}, {1,0, 0}, {1,-1, 0}, {0,1, 0}, {0,-1, 0}, {-1,1, 0}, {-1,0, 0}, {-1,-1, 0},
        {1,1, 1}, {1,0, 1}, {1,-1, 1}, {0,1, 1}, {0,-1, 1}, {-1,1, 1}, {-1,0, 1}, {-1,-1, 1}, {0,0, 1},
        {1,1,-1}, {1,0,-1}, {1,-1,-1}, {0,1,-1}, {0,-1,-1}, {-1,1,-1}, {-1,0,-1}, {-1,-1,-1}, {0,0,-1}
};

const std::vector<std::tuple<int, int, int, int>> directions4D = {
        {1,1, 0, 0}, {1,0, 0, 0}, {1,-1, 0, 0}, {0,1, 0, 0}, {0,-1, 0, 0}, {-1,1, 0, 0}, {-1,0, 0, 0}, {-1,-1, 0, 0},
        {1,1, 1, 0}, {1,0, 1, 0}, {1,-1, 1, 0}, {0,1, 1, 0}, {0,-1, 1, 0}, {-1,1, 1, 0}, {-1,0, 1, 0}, {-1,-1, 1, 0}, {0,0, 1, 0},
        {1,1,-1, 0}, {1,0,-1, 0}, {1,-1,-1, 0}, {0,1,-1, 0}, {0,-1,-1, 0}, {-1,1,-1, 0}, {-1,0,-1, 0}, {-1,-1,-1, 0}, {0,0,-1, 0},
        {1,1, 0,-1}, {1,0, 0,-1}, {1,-1, 0,-1}, {0,1, 0,-1}, {0,-1, 0,-1}, {-1,1, 0,-1}, {-1,0, 0,-1}, {-1,-1, 0,-1},
        {1,1, 1,-1}, {1,0, 1,-1}, {1,-1, 1,-1}, {0,1, 1,-1}, {0,-1, 1,-1}, {-1,1, 1,-1}, {-1,0, 1,-1}, {-1,-1, 1,-1}, {0,0, 1,-1},
        {1,1,-1,-1}, {1,0,-1,-1}, {1,-1,-1,-1}, {0,1,-1,-1}, {0,-1,-1,-1}, {-1,1,-1,-1}, {-1,0,-1,-1}, {-1,-1,-1,-1}, {0,0,-1,-1}, {0,0,0,-1},
        {1,1, 0, 1}, {1,0, 0, 1}, {1,-1, 0, 1}, {0,1, 0, 1}, {0,-1, 0, 1}, {-1,1, 0, 1}, {-1,0, 0, 1}, {-1,-1, 0, 1},
        {1,1, 1, 1}, {1,0, 1, 1}, {1,-1, 1, 1}, {0,1, 1, 1}, {0,-1, 1, 1}, {-1,1, 1, 1}, {-1,0, 1, 1}, {-1,-1, 1, 1}, {0,0, 1, 1},
        {1,1,-1, 1}, {1,0,-1, 1}, {1,-1,-1, 1}, {0,1,-1, 1}, {0,-1,-1, 1}, {-1,1,-1, 1}, {-1,0,-1, 1}, {-1,-1,-1, 1}, {0,0,-1, 1}, {0,0,0,1}
};

Coord3D::Coord3D(const Coord3D &other) {
  std::copy(std::begin(other.x_), std::end(other.x_), std::begin(x_));
}

Coord3D::Coord3D(const int &x, const int &y, const int &z) {
  x_[0] = x;
  x_[1] = y;
  x_[2] = z;
}

bool Coord3D::operator==(const Coord3D &other) const {
  return (x_[0] == other.x_[0]) &&
         (x_[1] == other.x_[1]) &&
         (x_[2] == other.x_[2]);
}

std::ostream& operator<<(std::ostream &os, const Coord3D &c) {
  return os << "(" << c.x_[0] << "," << c.x_[1] << "," << c.x_[2] << ")";
}

Coord4D::Coord4D(const Coord4D& other) {
  std::copy(std::begin(other.x_), std::end(other.x_), std::begin(x_));
}

Coord4D::Coord4D(const int &x, const int &y, const int &z, const int& w) {
  x_[0] = x;
  x_[1] = y;
  x_[2] = z;
  x_[3] = w;
}

bool Coord4D::operator==(const Coord4D &other) const {
  return (x_[0] == other.x_[0]) &&
         (x_[1] == other.x_[1]) &&
         (x_[2] == other.x_[2]) &&
         (x_[3] == other.x_[3]);
}

std::ostream& operator<<(std::ostream &os, const Coord4D &c) {
  return os << "(" << c.x_[0] << "," << c.x_[1] << "," << c.x_[2] << "," << c.x_[3] << ")";
}

void Grid3D::extend() {
  auto [minX, maxX, minY, maxY, minZ, maxZ] = this->bounds();
  for (int z = minZ-1; z <= maxZ+1; z++) {
    for (int x = minX-1; x <= maxX+1; x++) {
      for (int y = minY-1; y <= maxY+1; y++) {
        if (z >= minZ && z <= maxZ) {
          if (x >= minX && x <= maxX && y >= minY && y <= maxY) continue;
          this->inactive_.insert(Coord3D(x, y, z));
        } else {
          this->inactive_.insert(Coord3D(x, y, z));
        }
      } // y
    } // x
  } // z
}

std::tuple<int, int, int, int, int, int> Grid3D::bounds() const {
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

int Grid3D::nNeighbours(const Coord3D &coord) {
  int count = 0;
  for (const auto& [dx, dy, dz] : directions3D) {
    Coord3D adjacent(coord.x_[0]+dx,coord.x_[1]+dy,coord.x_[2]+dz);
    if (this->active_.find(adjacent) != this->active_.end()) count++;
  } // directions
  return count;
}

std::ostream& operator<<(std::ostream& os, const Grid3D& grid) {
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

void Grid4D::extend() {
  auto [minX, maxX, minY, maxY, minZ, maxZ, minW, maxW] = this->bounds();
  for (int w = minW-1; w <= maxW+1; w++) {
    for (int z = minZ-1; z <= maxZ+1; z++) {
      for (int x = minX-1; x <= maxX+1; x++) {
        for (int y = minY-1; y <= maxY+1; y++) {
          if (z >= minZ && z <= maxZ && w >= minW && w <= maxW) {
            if (x >= minX && x <= maxX && y >= minY && y <= maxY) continue;
            this->inactive_.insert(Coord4D(x, y, z, w));
          } else {
            this->inactive_.insert(Coord4D(x, y, z, w));
          }
        } // w
      } // z
    } // y
  } // x
}

std::tuple<int, int, int, int, int, int, int, int> Grid4D::bounds() const {
  int minX = 0, maxX = 0;
  int minY = 0, maxY = 0;
  int minZ = 0, maxZ = 0;
  int minW = 0, maxW = 0;
  for (const auto& c : active_) {
    if (c.x_[0] < minX) minX = c.x_[0];
    if (c.x_[0] > maxX) maxX = c.x_[0];
    if (c.x_[1] < minY) minY = c.x_[1];
    if (c.x_[1] > maxY) maxY = c.x_[1];
    if (c.x_[2] < minZ) minZ = c.x_[2];
    if (c.x_[2] > maxZ) maxZ = c.x_[2];
    if (c.x_[3] < minW) minW = c.x_[3];
    if (c.x_[3] > maxW) maxW = c.x_[3];
  }
  for (const auto& c : inactive_) {
    if (c.x_[0] < minX) minX = c.x_[0];
    if (c.x_[0] > maxX) maxX = c.x_[0];
    if (c.x_[1] < minY) minY = c.x_[1];
    if (c.x_[1] > maxY) maxY = c.x_[1];
    if (c.x_[2] < minZ) minZ = c.x_[2];
    if (c.x_[2] > maxZ) maxZ = c.x_[2];
    if (c.x_[3] < minW) minW = c.x_[3];
    if (c.x_[3] > maxW) maxW = c.x_[3];
  }
  return std::make_tuple(minX, maxX, minY, maxY, minZ, maxZ, minW, maxW);
}

int Grid4D::nNeighbours(const Coord4D &coord) {
  int count = 0;
  for (const auto& [dx, dy, dz, dw] : directions4D) {
    Coord4D adjacent(coord.x_[0]+dx,coord.x_[1]+dy,coord.x_[2]+dz, coord.x_[3]+dw);
    if (this->active_.find(adjacent) != this->active_.end()) count++;
  } // directions
  return count;
}