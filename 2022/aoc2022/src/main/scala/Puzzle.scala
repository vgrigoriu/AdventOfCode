trait Puzzle:
    def name: String = this.getClass.getSimpleName.replaceAll("\\$", "")
    def solve(): Int
