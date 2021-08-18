#include <iostream>

#include "ingredient.h"

using namespace advent_of_code;

int main() {
    auto const ingredients = get_ingredients();

    for (auto i: ingredients) {
        std::cout << i.name() << " has " << i.calories() << " calories." << "\n";
    }
}
