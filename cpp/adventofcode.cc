#include "adventofcode.h"

#include <string>

namespace adventofcode {
  answer_t ok(std::string msg) {
    answer_t a;
    a.answer = msg;
    return a;
  }

  answer_t err(std::string msg) {
    answer_t a;
    a.error = msg;
    return a;
  }
}
