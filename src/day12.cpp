#include <cctype>  // For std::isdigit
#include <fstream>
#include <iostream>
#include <string>
#include <vector>

struct SpringDetails {
  std::string condition_record;
  std::vector<int> groups;
};

SpringDetails parseDetails(std::string& line) {
  int condition_end = -1;

  for (int i = 0; i < line.length(); i++) {
    if (line[i] == ' ') {
      condition_end = i;
      break;
    }
  }

  std::vector<int> groups;
  int number_start = condition_end + 1;
  for (int i = number_start; i < line.length(); i++) {
    if (line[i] == ',') {
      groups.push_back(std::stoi(line.substr(number_start, i - number_start)));
      number_start = i + 1;
    }
  }
  groups.push_back(
      std::stoi(line.substr(number_start, line.length() - number_start)));

  return SpringDetails{line.substr(0, condition_end), groups};
}

int main() {
  std::ifstream input_file("./inputs/day12.txt");

  if (!input_file) {
    return 1;
  }

  std::string line;
  std::vector<SpringDetails> springs;
  while (std::getline(input_file, line)) {
    springs.push_back(parseDetails(line));
  }

  input_file.close();
  return 0;
}
