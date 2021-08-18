#include "ingredient.h"

namespace advent_of_code {

std::vector<Ingredient> get_ingredients() {
  return {
    // name      capacity durability flavor texture calories
    { "Sprinkles",      5,        -1,     0,      0,       5 },
    { "Peanut Butter", -1,         3,     0,      0,       1 },
    { "Frosting",       0,        -1,     4,      0,       6 },
    { "Sugar",         -1,         0,     0,      2,       8 }
  };
}

}
