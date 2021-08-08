#include <algorithm>
#include <array>
#include <iostream>
#include <limits>
#include <vector>

namespace {
    enum People {
        Alice,
        Bob,
        Carol,
        David,
        Eric,
        Frank,
        George,
        Mallory,
        Me
    };

    int total_happiness(std::vector<People> people, std::array<std::array<int, 8>, 8> happiness) {
        int result = 0;
        for (int i = 0; i < people.size(); ++i) {
            People p = people[i];
            People next_p = people[(i + 1) % people.size()];

            if (p != Me && next_p != Me) {
                result += happiness[p][next_p];
                result += happiness[next_p][p];
            }
        }

        return result;
    }
}

int main() {
    std::array<std::array<int, 8>, 8> happiness {};

    happiness[Alice][Bob] = 2;
    happiness[Alice][Carol] = 26;
    happiness[Alice][David] = -82;
    happiness[Alice][Eric] = -75;
    happiness[Alice][Frank] = 42;
    happiness[Alice][George] = 38;
    happiness[Alice][Mallory] = 39;
    happiness[Bob][Alice] = 40;
    happiness[Bob][Carol] = -61;
    happiness[Bob][David] = -15;
    happiness[Bob][Eric] = 63;
    happiness[Bob][Frank] = 41;
    happiness[Bob][George] = 30;
    happiness[Bob][Mallory] = 87;
    happiness[Carol][Alice] = -35;
    happiness[Carol][Bob] = -99;
    happiness[Carol][David] = -51;
    happiness[Carol][Eric] = 95;
    happiness[Carol][Frank] = 90;
    happiness[Carol][George] = -16;
    happiness[Carol][Mallory] = 94;
    happiness[David][Alice] = 36;
    happiness[David][Bob] = -18;
    happiness[David][Carol] = -65;
    happiness[David][Eric] = -18;
    happiness[David][Frank] = -22;
    happiness[David][George] = 2;
    happiness[David][Mallory] = 42;
    happiness[Eric][Alice] = -65;
    happiness[Eric][Bob] = 24;
    happiness[Eric][Carol] = 100;
    happiness[Eric][David] = 51;
    happiness[Eric][Frank] = 21;
    happiness[Eric][George] = 55;
    happiness[Eric][Mallory] = -44;
    happiness[Frank][Alice] = -48;
    happiness[Frank][Bob] = 91;
    happiness[Frank][Carol] = 8;
    happiness[Frank][David] = -66;
    happiness[Frank][Eric] = 97;
    happiness[Frank][George] = -9;
    happiness[Frank][Mallory] = -92;
    happiness[George][Alice] = -44;
    happiness[George][Bob] = -25;
    happiness[George][Carol] = 17;
    happiness[George][David] = 92;
    happiness[George][Eric] = -92;
    happiness[George][Frank] = 18;
    happiness[George][Mallory] = 97;
    happiness[Mallory][Alice] = 92;
    happiness[Mallory][Bob] = -96;
    happiness[Mallory][Carol] = -51;
    happiness[Mallory][David] = -81;
    happiness[Mallory][Eric] = 31;
    happiness[Mallory][Frank] = -73;
    happiness[Mallory][George] = -89;

    std::vector<People> people{Alice, Bob, Carol, David, Eric, Frank, George, Mallory, Me};

    int max = std::numeric_limits<int>::min();

    do {
        int h = total_happiness(people, happiness);
        if (max < h) {
            max = h;
        }
    } while (std::next_permutation(people.begin(), people.end()));

    std::cout << max << "\n";

    return 0;
}
