trait Puzzle:
    def name: String = this.getClass.getSimpleName.replaceAll("\\$|\\d", "")
    def solve(): Int
