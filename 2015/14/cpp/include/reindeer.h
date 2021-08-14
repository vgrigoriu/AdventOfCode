#ifndef REINDEER_H
#define REINDEER_H

#include <iostream>
#include <string>

namespace advent_of_code {
class Reindeer {
  std::string name;
  int speed;
  int fly_time;
  int rest_time;
  int score{0};

public:
  int distance_after(int seconds) const;
  void award_point() { ++score; };
  int get_score() const { return score; };

  friend auto operator<<(std::ostream &os, const Reindeer &dt)
      -> std::ostream &;
  friend auto operator>>(std::istream &os, Reindeer &dt) -> std::istream &;
};
} // namespace advent_of_code

#endif
