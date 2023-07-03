#include <algorithm>
#include <array>
#include <fstream>
#include <iostream>
#include <string>

void part1();

void part2();

int main() {
  part1();
  part2();
  return 0;
}

void part1() {
  std::ifstream fin("input");

  size_t calories = 0;
  std::string buffer;
  size_t max_calories = 0;

  while (std::getline(fin, buffer)) {
    if (buffer == "") {
      max_calories = std::max(max_calories, calories);
      calories = 0;
    } else {
      calories += std::stoi(buffer);
    }
  }

  std::cout << "part1:\n" << max_calories << std::endl;
}

void part2() {
  std::ifstream fin("input");

  size_t calories = 0;
  std::string buffer;
  std::array<size_t, 3> top3 = {0, 0, 0};

  while (std::getline(fin, buffer)) {
    if (buffer == "") {
      if (calories > top3[0]) {
        top3[0] = calories;
        if (top3[0] > top3[1]) {
          std::swap(top3[0], top3[1]);
        } else if (top3[0] > top3[2]) {
          std::swap(top3[0], top3[2]);
        }
      }
      calories = 0;
    } else {
      calories += std::stoi(buffer);
    }
  }

  size_t sum = top3[0] + top3[1] + top3[2];
  std::cout << "part2:\n" << sum << std::endl;
}
