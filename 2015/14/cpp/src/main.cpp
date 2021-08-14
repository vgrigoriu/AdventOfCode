#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

#include "reindeer.h"

using namespace std;
using namespace advent_of_code;

namespace {
void award_points_after(int seconds, vector<Reindeer> &reindeers) {
  vector<int> distances{};
  transform(reindeers.cbegin(), reindeers.cend(), back_inserter(distances),
            [seconds](auto &r) { return r.distance_after(seconds); });

  auto max_distance = max_element(distances.cbegin(), distances.cend());

  for_each(reindeers.begin(), reindeers.end(),
           [seconds, max_distance](auto &r) {
             if (r.distance_after(seconds) == *max_distance) {
               r.award_point();
             }
           });
}
} // namespace

auto main(int argc, char **argv) -> int {
  if (argc != 2) {
    cout << "Usage: " << argv[0] << " path/to/input"
         << "\n";
    return 1;
  }

  ifstream in{argv[1]};
  vector<Reindeer> reindeers{};

  copy(istream_iterator<Reindeer>{in}, istream_iterator<Reindeer>{},
       back_inserter(reindeers));

  vector<int> distances{};
  transform(reindeers.cbegin(), reindeers.cend(), back_inserter(distances),
            [](auto &r) { return r.distance_after(2503); });

  auto max_distance = max_element(distances.cbegin(), distances.cend());

  cout << "Maximum distance is " << *max_distance << "\n";

  for (int s = 1; s <= 2503; ++s) {
    award_points_after(s, reindeers);
  }

  auto max_score_reindeer =
      max_element(reindeers.cbegin(), reindeers.cend(), [](auto &r1, auto &r2) {
        return r1.get_score() < r2.get_score();
      });

  cout << "Maximum score is " << max_score_reindeer->get_score() << "\n";
  return 0;
}
