#include <fstream>
#include <iostream>
#include <utility>

typedef struct Interval {
  int start;
  int end;
} Interval;

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

  size_t count = 0;
  char _;
  size_t start1, end1, start2, end2;
  while (fin >> start1 >> _ >> end1 >> _ >> start2 >> _ >> end2) {
#ifdef DEBUG
    std::cout << start1 << "-" << end1 << "," << start2 << "-" << end2
              << std::endl;
#endif
    if ((start1 <= start2 && end2 <= end1) ||
        (start2 <= start1 && end1 <= end2)) {
      count++;
    }
  }

  std::cout << "part1: \n" << count << std::endl;
}

void part2(std::string filename) {
  std::ifstream fin(filename);

  size_t count = 0;
  char _;
  size_t start1, end1, start2, end2;
  while (fin >> start1 >> _ >> end1 >> _ >> start2 >> _ >> end2) {
#ifdef DEBUG
    std::cout << start1 << "-" << end1 << "," << start2 << "-" << end2
              << std::endl;
#endif
    if ((start1 <= start2 && start2 <= end1) ||
        (start2 <= start1 && start1 <= end2)) {
      count++;
    }
  }

  std::cout << "part2: \n" << count << std::endl;
}
