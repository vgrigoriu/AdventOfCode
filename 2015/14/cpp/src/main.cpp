#include <algorithm>
#include <fstream>
#include <iostream>
#include <vector>

#include "reindeer.h"

using namespace std;
using namespace advent_of_code;

auto main(int argc, char **argv) -> int {
    if (argc != 2) {
        cout << "Usage: " << argv[0] << " path/to/input" << "\n";
        return 1;
    }
    
    ifstream in{argv[1]};
    vector<Reindeer> reindeers{};

    copy(istream_iterator<Reindeer>{in},
         istream_iterator<Reindeer>{},
         back_inserter(reindeers));

    vector<int> distances{};
    transform(reindeers.cbegin(),
              reindeers.cend(),
              back_inserter(distances),
              [](auto& r) {return r.distance_after(2503);});

    for (auto d: distances) {
        cout << "Distance: " << d << "\n";
    }

    auto max_distance = max_element(distances.cbegin(), distances.cend());

    cout << "Maximum distance is " << *max_distance << "\n";

    return 0;
}
