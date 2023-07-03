#include <fstream>
#include <iostream>
#include <tuple>

enum Shape {
  ROCK = 0,
  PAPER = 1,
  SCISSORS = 2,
};

void part1();

void part2();

std::tuple<bool, Shape, Shape> parse_line(std::ifstream &file);

int main() {
  part1();
  part2();
  return 0;
}

void part1() {
  std::ifstream fin("input");
  size_t score = 0;

  for (auto [success, s1, s2] = parse_line(fin); success;
       std::tie(success, s1, s2) = parse_line(fin)) {
    score += s2 + 1;
    score += (s2 + 4 - s1) % 3 * 3;
  }

  std::cout << "part1:\n" << score << std::endl;
}

void part2() {
  std::ifstream fin("input");
  size_t score = 0;

  for (auto [success, s1, s2] = parse_line(fin); success;
       std::tie(success, s1, s2) = parse_line(fin)) {
    score += s2 * 3;
    score += (s2 + s1 + 2) % 3 + 1;
  }

  std::cout << "part2:\n" << score << std::endl;
}

std::tuple<bool, Shape, Shape> parse_line(std::ifstream &file) {
  std::string s;
  Shape shape1, shape2;
  if (!(file >> s)) {
    return {false, ROCK, ROCK};
  }
  switch (s[0]) {
  case 'A':
    shape1 = ROCK;
    break;
  case 'B':
    shape1 = PAPER;
    break;
  case 'C':
    shape1 = SCISSORS;
    break;
  }
  file >> s;
  switch (s[0]) {
  case 'X':
    shape2 = ROCK;
    break;
  case 'Y':
    shape2 = PAPER;
    break;
  case 'Z':
    shape2 = SCISSORS;
    break;
  }
  return {true, shape1, shape2};
}
