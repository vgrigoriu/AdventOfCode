import scala.collection.mutable.Stack

private trait SupplyStacksBase:
    protected def getStacks(stacksInput: Seq[String]): Seq[Stack[Char]] =
        Seq(
          Stack("HRBDZFLS": _*),
          Stack("TMBZR": _*),
          Stack("ZLCHNS": _*),
          Stack("SCFJ": _*),
          Stack("PGHWRZB": _*),
          Stack("VJZGDNMT": _*),
          Stack("GLNWFSPQ": _*),
          Stack("MZR": _*),
          Stack("MCLGVRT": _*),
        ).map(_.reverse)

    protected def getMoves(movesInput: Seq[String]): Seq[Move] =
        movesInput.map(Move.apply)

object SupplyStacks extends StringPuzzle, SupplyStacksBase:
    override def solve(): String =
        val input                        = readInput()
        val Seq(stacksInput, movesInput) = input.splitBy(_.isEmpty())
        val stacks                       = getStacks(stacksInput)
        val moves                        = getMoves(movesInput)

        moves.foreach(move => moveOneByOne(move, stacks))

        stacks.map(_.top).mkString

object SupplyStacks2 extends StringPuzzle, SupplyStacksBase:
    override def solve(): String =
        val input                        = readInput()
        val Seq(stacksInput, movesInput) = input.splitBy(_.isEmpty())
        val stacks                       = getStacks(stacksInput)
        val moves                        = getMoves(movesInput)

        moves.foreach(move => moveAllAtOnce(move, stacks))

        stacks.map(_.top).mkString

case class Move(howMany: Int, from: Int, to: Int)

object Move:
    def apply(input: String): Move =
        val Array(howMany, from, to) = input.substring(5).split(" [a-z]+ ")
        Move(howMany.toInt, from.toInt - 1, to.toInt - 1)

private def moveOneByOne(move: Move, stacks: Seq[Stack[Char]]): Unit =
    for i <- 1 to move.howMany do stacks(move.to).push(stacks(move.from).pop())

private def moveAllAtOnce(move: Move, stacks: Seq[Stack[Char]]): Unit =
    val temp = Stack[Char]()
    for i <- 1 to move.howMany do temp.push(stacks(move.from).pop())
    for i <- 1 to move.howMany do stacks(move.to).push(temp.pop())
