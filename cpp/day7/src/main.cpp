#include <fstream>
#include <iostream>

void part1(std::string filename);

void part2(std::string filename);

int main() {
#ifdef DEBUG
  part1("input_test");
  part2("input_test");
#else
  part1("input");
  part2("input");
#endif
  return 0;
}

void part1(std::string filename) {
  std::ifstream fin(filename);

  std::cout << "part1: \n" << 0 << std::endl;
  fin.close();
}

void part2(std::string filename) {
  std::ifstream fin(filename);

  std::cout << "part2: \n" << 0 << std::endl;
  fin.close();
}
