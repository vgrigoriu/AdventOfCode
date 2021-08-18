#include <iostream>

#include "ingredient.h"

using namespace advent_of_code;

int main() {
    auto const ingredients = get_ingredients();

    int const max_ingredients = 100;
    int max_score = 0;
    for (int i1 = 0; i1 <= max_ingredients; ++i1) {
        for (int i2 = 0; i1 + i2 <= max_ingredients; ++i2) {
            for (int i3 = 0; i1 + i2 + i3 <= max_ingredients; ++i3) {
                int i4 = max_ingredients - (i1 + i2 + i3);
                int score = compute_score(ingredients, i1, i2, i3, i4);
                if (max_score < score) {
                    max_score = score;
                }
            }
        }
    }

    std::cout << "Max score is " << max_score << "\n";

    int const target_calories = 500;
    max_score = 0;
    for (int i1 = 0; i1 <= max_ingredients; ++i1) {
        for (int i2 = 0; i1 + i2 <= max_ingredients; ++i2) {
            for (int i3 = 0; i1 + i2 + i3 <= max_ingredients; ++i3) {
                int i4 = max_ingredients - (i1 + i2 + i3);
                int calories = compute_calories(ingredients, i1, i2, i3, i4);
                if (calories != target_calories) {
                    continue;
                }
                int score = compute_score(ingredients, i1, i2, i3, i4);
                if (max_score < score) {
                    max_score = score;
                }
            }
        }
    }

    std::cout << "Max score for 500 calories is " << max_score << "\n";
}
