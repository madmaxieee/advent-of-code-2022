#include <fstream>
#include <iostream>
#include <stack>
#include <vector>

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
  std::vector<std::stack<char>> stacks;
  std::vector<std::stack<char>> data;

  std::string line;
  while (std::getline(fin, line)) {
    if (line[1] == '1') {
      break;
    }
    for (size_t i = 1; i < line.size(); i += 4) {
      size_t ind = i / 4;
      if (ind >= data.size()) {
        data.push_back(std::stack<char>());
      }
      if (line[i] != ' ') {
        data[ind].push(line[i]);
      }
    }
#ifdef DEBUG
    std::cout << line << std::endl;
#endif
  }

  for (auto &s : data) {
    std::stack<char> tmp;
    while (!s.empty()) {
      tmp.push(s.top());
      s.pop();
    }
    stacks.push_back(tmp);
  }

  std::string _;
  size_t n, src, dest;
  while (fin >> _ >> n >> _ >> src >> _ >> dest) {
    for (size_t i = 0; i < n; ++i) {
      stacks[dest - 1].push(stacks[src - 1].top());
      stacks[src - 1].pop();
    }
  }

  std::cout << "part1:" << std::endl;
  for (auto &s : stacks) {
    std::cout << s.top();
  }
  std::cout << std::endl;
}

void part2(std::string filename) {
  std::ifstream fin(filename);
  std::vector<std::stack<char>> stacks;
  std::vector<std::stack<char>> data;

  std::string line;
  while (std::getline(fin, line)) {
    if (line[1] == '1') {
      break;
    }
    for (size_t i = 1; i < line.size(); i += 4) {
      size_t ind = i / 4;
      if (ind >= data.size()) {
        data.push_back(std::stack<char>());
      }
      if (line[i] != ' ') {
        data[ind].push(line[i]);
      }
    }
#ifdef DEBUG
    std::cout << line << std::endl;
#endif
  }

  for (auto &s : data) {
    std::stack<char> tmp;
    while (!s.empty()) {
      tmp.push(s.top());
      s.pop();
    }
    stacks.push_back(tmp);
  }

  std::string _;
  size_t n, src, dest;
  while (fin >> _ >> n >> _ >> src >> _ >> dest) {
    std::stack<char> tmp;
    for (size_t i = 0; i < n; ++i) {
      tmp.push(stacks[src - 1].top());
      stacks[src - 1].pop();
    }
    while (!tmp.empty()) {
      stacks[dest - 1].push(tmp.top());
      tmp.pop();
    }
  }

  std::cout << "part2:" << std::endl;
  for (auto &s : stacks) {
    std::cout << s.top();
  }
  std::cout << std::endl;
}
