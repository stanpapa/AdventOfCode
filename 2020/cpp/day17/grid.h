#include <unordered_set>
#include <tuple>
#include <iostream>

struct Coord3D {
  int x_[3] = {0};

  Coord3D() = default;
  Coord3D(const Coord3D &other);
  Coord3D(const int &x, const int &y, const int &z);

  bool operator==(const Coord3D &other) const;
  friend std::ostream &operator<<(std::ostream &os, const Coord3D &c);
};

struct Coord4D {
  int x_[4] = {0};

  Coord4D() = default;
  Coord4D(const Coord4D &other);
  Coord4D(const int &x, const int &y, const int &z, const int& w);

  bool operator==(const Coord4D &other) const;
  friend std::ostream &operator<<(std::ostream &os, const Coord4D &c);
};

template <>
struct std::hash<Coord3D> {
  size_t operator()( const Coord3D& toHash) const {
      size_t hash = 0;
      for (const int& i : toHash.x_) hash += std::hash<int>{}(i);
      return hash;
  };
};

template <>
struct std::hash<Coord4D> {
  size_t operator()(const Coord4D &toHash) const {
    size_t hash = 0;
    for (const int &i : toHash.x_) hash += std::hash<int>{}(i);
    return hash;
  };
};

struct Grid3D {
    std::unordered_set<Coord3D, std::hash<Coord3D>> active_;
    std::unordered_set<Coord3D, std::hash<Coord3D>> inactive_;

    Grid3D() = default;
    Grid3D(const Grid3D &other);

    int nActive() const;
    int nInactive() const;
    int size() const;

    void clear();

    std::tuple<int, int, int, int, int, int> xyzBounds() const;

    Grid3D& operator()(const Grid3D &other);
    friend std::ostream &operator<<(std::ostream &os, const Grid3D &grid);
};