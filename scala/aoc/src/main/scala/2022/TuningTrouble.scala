object TuningTrouble extends IntPuzzle:
    override def solve(): Int =
        val input = readInput()(0)
        findFirstDistinctRun(input, 4)

object TuningTrouble2 extends IntPuzzle:
    override def solve(): Int =
        val input = readInput()(0)
        findFirstDistinctRun(input, 14)

private def findFirstDistinctRun(input: String, size: Int): Int =
    (0 until input.length - size).find(i =>
        input.substring(i, i + size).toSet.size == size,
    ).get + size
