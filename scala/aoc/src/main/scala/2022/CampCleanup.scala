object CampCleanup extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val assignemnts = input.map(Assignments.apply)

        assignemnts.filter(_.oneContainsTheOther).length

object CampCleanup2 extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val assignemnts = input.map(Assignments.apply)

        assignemnts.filter(_.overlaps).length

case class Assignments(val first: Range, val second: Range):
    def oneContainsTheOther: Boolean =
        first.fullyContains(second) || second.fullyContains(first)
    def overlaps: Boolean =
        first.overlaps(second)

object Assignments:
    // 28-47,45-47
    def apply(input: String): Assignments =
        val ranges = input
            .split(",")
            .map(_.split("-"))
            .map(_.map(_.toInt))
            .map(pair => Range.inclusive(pair(0), pair(1)))
        Assignments(ranges(0), ranges(1))
