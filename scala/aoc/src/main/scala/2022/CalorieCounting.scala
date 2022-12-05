trait CalorieCountingBase extends IntPuzzle:
    protected class Calories(valuesAsStrings: Seq[String]):
        val values = valuesAsStrings.map(_.toInt)
        val total  = values.sum

    protected def getCaloriesPerElf(): Seq[Calories] =
        val input  = readInput()
        val groups = input.splitBy(_.isEmpty)

        groups.map(Calories(_))

object CalorieCounting extends CalorieCountingBase:
    def solve(): Int =
        getCaloriesPerElf().map(_.total).max

object CalorieCounting2 extends CalorieCountingBase:
    def solve(): Int =
        getCaloriesPerElf().map(_.total).sorted.reverse.take(3).sum
