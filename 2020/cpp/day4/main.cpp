#include "../utils.h"
#include <regex>

bool validYear(const std::string& s, const int& min, const int& max) {
  if (s.size() != 4) return false;
  int year = std::stoi(s);
  if (year < min || year > max) return false;
  return true;
}

struct Passport {
  std::string byr_ = "";
  std::string iyr_ = "";
  std::string eyr_ = "";
  std::string hgt_ = "";
  std::string hcl_ = "";
  std::string ecl_ = "";
  std::string pid_ = "";
  std::string cid_ = "";

  Passport(const std::string& str);

  bool isValid1() const;
  bool isValid2() const;

  friend std::ostream& operator<<(std::ostream& os, const Passport& p);
};

Passport::Passport(const std::string& str) {
  std::vector<std::string> fields = split(str);
  for (const std::string& field : fields) {
    std::vector<std::string> pair = split(field, ":");
    if (pair[0] == "byr") {
      byr_ = pair[1];
    } else if (pair[0] == "iyr") {
      iyr_ = pair[1];
    } else if (pair[0] == "eyr") {
      eyr_ = pair[1];
    } else if (pair[0] == "hgt") {
      hgt_ = pair[1];
    } else if (pair[0] == "hcl") {
      hcl_ = pair[1];
    } else if (pair[0] == "ecl") {
      ecl_ = pair[1];
    } else if (pair[0] == "pid") {
      pid_ = pair[1];
    } else if (pair[0] == "cid") {
      cid_ = pair[1];
    }
  } // field
}

bool Passport::isValid1() const {
  if (byr_.empty() || iyr_.empty() || eyr_.empty() ||
      hgt_.empty() || hcl_.empty() || ecl_.empty() ||
      pid_.empty()) {
    return false;
  } else {
    return true;
  }
}

bool Passport::isValid2() const {
  if (!isValid1()) return false; // are fields present?

  // byr
  if (!validYear(byr_, 1920, 2002)) return false;

  // iyr
  if (!validYear(iyr_, 2010, 2020)) return false;

  // eyr
  if (!validYear(eyr_, 2020, 2030)) return false;

  // hgt
  {
    std::string unit = hgt_.substr(hgt_.size()-2,2);
    if (!(unit == "cm" || unit == "in")) return false;
    int length = std::stoi(hgt_.substr(0,hgt_.size()-2));
    if ((unit == "cm" && !(length >= 150 && length <= 193)) ||
        (unit == "in" && !(length >= 59 && length <= 76))) {
      return false;
    }
  }

  // hcl
  {
    const std::regex allowedAlphaNum(R"(#[a-f0-9]{6})"); 
    if (!std::regex_match(hcl_, allowedAlphaNum)) return false;
  }

  // ecl
  if (!(ecl_ == "amb" || ecl_ == "blu" || ecl_ == "brn" || ecl_ == "gry" ||
        ecl_ == "grn" || ecl_ == "hzl" || ecl_ == "oth")) return false;

  // pid
  if (pid_.size() != 9 || !isNumber(pid_)) return false;

  return true;
}

std::ostream& operator<<(std::ostream& os, const Passport& p) {
  os << "byr: " << p.byr_ << "\n";
  os << "iyr: " << p.iyr_ << "\n";
  os << "eyr: " << p.eyr_ << "\n";
  os << "hgt: " << p.hgt_ << "\n";
  os << "hcl: " << p.hcl_ << "\n";
  os << "ecl: " << p.ecl_ << "\n";
  os << "pid: " << p.pid_ << "\n";
  os << "cid: " << p.cid_ << "\n";
  return os;
}

int main(int argc, char* argv[]) {

  if (argc != 2) {
    std::cerr << "input file is missing! ABORT.\n";
    exit(1);
  }

  // read input and create raw passports
  std::vector<std::string> input;
  readInputAndSplit(std::string(argv[1]), input);

  // convert raw passports into proper ones
  std::vector<Passport> passports; 
  passports.reserve(input.size());
  int count = 0;
  for (const std::string& s : input) {
    passports.push_back(Passport(s));
  }

  // part 1
  int nValid = 0;
  for (const Passport& p : passports) {
    if (p.isValid1()) nValid++;
  }
  std::cout << "Answer Part 1: " << nValid << "\n";

  // part 2
  nValid = 0;
  for (const Passport& p : passports) {
    if (p.isValid2()) nValid++;
  }
  std::cout << "Answer Part 2: " << nValid << "\n";


  return 0;
} 
