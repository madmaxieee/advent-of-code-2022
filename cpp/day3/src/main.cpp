#include <fstream>
#include <iostream>

void part1(std::string file_name);

void part2(std::string file_name);

uint8_t get_index(char c) {
  if (c >= 'a' && c <= 'z') {
    return c - 'a';
  } else if (c >= 'A' && c <= 'Z') {
    return c - 'A' + 26;
  }

  return 0;
}

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
  std::bitset<52> flags1;
  std::bitset<52> flags2;

  std::ifstream fin(filename);

  size_t total = 0;
  std::string line;
  while (fin >> line) {
    flags1 = std::bitset<52>{0};
    flags2 = std::bitset<52>{0};

    size_t l = line.size() / 2;
    for (size_t i = 0; i < l; i++) {
      uint8_t index1 = get_index(line[i]);
      uint8_t index2 = get_index(line[i + l]);

      if (flags2.test(index1)) {
        total += index1 + 1;
#ifdef DEBUG
        std::cout << line[i] << std::endl;
#endif
        goto next_line;
      } else {
        flags1.set(index1);
      }

      if (flags1.test(index2)) {
        total += index2 + 1;
#ifdef DEBUG
        std::cout << line[i + l] << std::endl;
#endif
        goto next_line;
      } else {
        flags2.set(index2);
      }
    }

  next_line:
    continue;
  }

  std::cout << "part1: \n" << total << std::endl;
}

void part2(std::string filename) {
  std::bitset<52> flags1;
  std::bitset<52> flags2;
  std::ifstream fin(filename);

  size_t total = 0;
  std::string line1, line2, line3;
  while (fin >> line1 >> line2 >> line3) {
    flags1 = std::bitset<52>{0};
    flags2 = std::bitset<52>{0};

    for (char c : line1) {
      flags1.set(get_index(c));
    }
    for (char c : line2) {
      flags2.set(get_index(c));
    }
    flags1 &= flags2;
    for (char c : line3) {
      if (flags1.test(get_index(c))) {
        total += get_index(c) + 1;
        goto next_group;
      }
    }

  next_group:
    continue;
  }

  std::cout << "part2: \n" << total << std::endl;
}
