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

template <class T>
struct Grid {
  std::unordered_set<T, std::hash<T>> active_;
  std::unordered_set<T, std::hash<T>> inactive_;

  Grid() = default;
  Grid(const Grid<T> &other) {
    active_ = other.active_;
    inactive_ = other.inactive_;
  }

  int nActive() const { return active_.size(); }
  int nInactive() const { return inactive_.size(); }
  int size() const { return active_.size() + inactive_.size(); }

  void clear() { active_.clear(); inactive_.clear(); };

  Grid<T>& operator()(const Grid<T> &other) {
    active_ = other.active_;
    inactive_ = other.inactive_;
    return *this;
  }
};

struct Grid3D : Grid<Coord3D> {
    void extend();
    std::tuple<int, int, int, int, int, int> bounds() const;
    int nNeighbours(const Coord3D& coord);

    friend std::ostream& operator<<(std::ostream& os, const Grid3D& grid);
};

struct Grid4D : Grid<Coord4D> {
    void extend();
    std::tuple<int, int, int, int, int, int, int, int> bounds() const;
    int nNeighbours(const Coord4D& coord);
};