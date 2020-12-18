#include "grid.h"

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

Grid3D::Grid3D(const Grid3D& other) {
  active_ = other.active_;
  inactive_ = other.inactive_;
}

int Grid3D::nActive() const { return active_.size(); }
int Grid3D::nInactive() const { return inactive_.size(); }
int Grid3D::size() const { return active_.size() + inactive_.size(); }

void Grid3D::clear() { active_.clear(); inactive_.clear(); }

std::tuple<int, int, int, int, int, int> Grid3D::xyzBounds() const {
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

Grid3D& Grid3D::operator()(const Grid3D& other) {
  active_ = other.active_;
  inactive_ = other.inactive_;
  return *this;
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