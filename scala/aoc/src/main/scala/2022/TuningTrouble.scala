object TuningTrouble extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        findFirstDistinctRun(input(0), 4)

object TuningTrouble2 extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        findFirstDistinctRun(input(0), 14)

private def findFirstDistinctRun(input: String, size: Int): Int =
    (0 until input.length - size).find(i =>
        input.substring(i, i + size).toSet.size == size,
    ).get + size
