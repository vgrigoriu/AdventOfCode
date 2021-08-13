#include "reindeer.h"

namespace advent_of_code {

std::ostream& operator<<(std::ostream& os, const Reindeer& r)
{
    os << "Hi, I'm " << r.name << "! ";
    os << "I can fly " << r.speed << " km/s for " << r.fly_time << " seconds, ";
    os << "but then I must rest for " << r.rest_time << " seconds.\n";
    return os;
}

std::istream& operator>>(std::istream& is, Reindeer& r)
{
    // Donner can fly 9 km/s for 5 seconds, but then must rest for 38 seconds.
    std::string skip;
    //    Donner    can     fly     9          km/s    for     5
    is >> r.name >> skip >> skip >> r.speed >> skip >> skip >> r.fly_time;
    //    seconds, but     then    must    rest    for     38             seconds.
    is >> skip  >> skip >> skip >> skip >> skip >> skip >> r.rest_time >> skip;
    return is;
}

}
