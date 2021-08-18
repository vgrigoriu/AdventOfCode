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

int compute_score(std::vector<Ingredient> const ingredients, int i1, int i2, int i3, int i4) {
    int capacity = i1 * ingredients[0].capacity()
                 + i2 * ingredients[1].capacity()
                 + i3 * ingredients[2].capacity()
                 + i4 * ingredients[3].capacity();
    int durability = i1 * ingredients[0].durability()
                   + i2 * ingredients[1].durability()
                   + i3 * ingredients[2].durability()
                   + i4 * ingredients[3].durability();
    int flavor = i1 * ingredients[0].flavor()
               + i2 * ingredients[1].flavor()
               + i3 * ingredients[2].flavor()
               + i4 * ingredients[3].flavor();
    int texture = i1 * ingredients[0].texture()
                + i2 * ingredients[1].texture()
                + i3 * ingredients[2].texture()
                + i4 * ingredients[3].texture();

    if (capacity < 0 || durability < 0 || flavor < 0 || texture < 0) {
        return 0;
    }

    return capacity * durability * flavor * texture;
}

int compute_calories(std::vector<Ingredient> const ingredients, int i1, int i2, int i3, int i4) {
    return i1 * ingredients[0].calories()
         + i2 * ingredients[1].calories()
         + i3 * ingredients[2].calories()
         + i4 * ingredients[3].calories();
}

}
