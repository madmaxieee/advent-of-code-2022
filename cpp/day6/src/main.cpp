#include <_types/_uint32_t.h>
#include <array>
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
  uint32_t flags = 0;

  std::string stream;
  fin >> stream;
  fin.close();

  for (size_t i = 0; i < stream.size() - 4; i++) {
    flags = 0;
    for (size_t j = i; j < i + 4; j++) {
      if (flags & (1 << (stream[j] - 'a'))) {
        goto next;
      } else {
        flags |= 1 << (stream[j] - 'a');
      }
    }

    std::cout << "part1: \n" << i + 4 << std::endl;
    return;

  next:
    continue;
  }
}

void part2(std::string filename) {
  std::ifstream fin(filename);
  uint32_t flags = 0;

  std::string stream;
  fin >> stream;
  fin.close();

  for (size_t i = 0; i < stream.size() - 14; i++) {
    flags = 0;
    for (size_t j = i; j < i + 14; j++) {
      if (flags & (1 << (stream[j] - 'a'))) {
        goto next;
      } else {
        flags |= 1 << (stream[j] - 'a');
      }
    }

    std::cout << "part2: \n" << i + 14 << std::endl;
    return;

  next:
    continue;
  }
}
