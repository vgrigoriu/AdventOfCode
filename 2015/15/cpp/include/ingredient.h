#include <string>
#include <vector>

namespace advent_of_code {

class Ingredient {
  std::string name_;
  int capacity_;
  int durability_;
  int flavor_;
  int texture_;
  int calories_;

public:
  Ingredient(std::string name, int c, int d, int f, int t, int cal)
      : name_{name}, capacity_{c},
        durability_{d}, flavor_{f}, texture_{t}, calories_{cal} {}

  std::string name() const { return name_; }
  int capacity() const { return capacity_; }
  int durability() const { return durability_; }
  int flavor() const { return flavor_; }
  int texture() const { return texture_; }
  int calories() const { return calories_; }

};

std::vector<Ingredient> get_ingredients();

int compute_score(std::vector<Ingredient> const ingredients, int i1, int i2, int i3, int i4);
int compute_calories(std::vector<Ingredient> const ingredients, int i1, int i2, int i3, int i4);

} // namespace advent_of_code
