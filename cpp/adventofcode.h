#ifndef ADVENTOFCODE_ADVENTOFCODE_H
#define ADVENTOFCODE_ADVENTOFCODE_H

#include <string>

namespace adventofcode {
  typedef struct {
    std::string answer;
    std::string error;
  } answer_t;

  answer_t ok(std::string msg);
  answer_t err(std::string msg);
}

#endif
