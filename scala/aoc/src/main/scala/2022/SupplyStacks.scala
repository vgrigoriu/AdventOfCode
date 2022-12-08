import scala.collection.mutable.Stack

private trait SupplyStacksBase:

    protected def getStacks(stacksInput: Seq[String]): Seq[Stack[Char]] =
        // [S]                 [T] [Q]
        // [L]             [B] [M] [P]     [T]
        // [F]     [S]     [Z] [N] [S]     [R]
        // [Z] [R] [N]     [R] [D] [F]     [V]
        // [D] [Z] [H] [J] [W] [G] [W]     [G]
        // [B] [M] [C] [F] [H] [Z] [N] [R] [L]
        // [R] [B] [L] [C] [G] [J] [L] [Z] [C]
        // [H] [T] [Z] [S] [P] [V] [G] [M] [M]
        //  1   2   3   4   5   6   7   8   9
        val maxLength = stacksInput.map(_.length).max

        stacksInput
            // lose last row
            .init
            // pad so they're all the same length
            .map(_.padTo(maxLength, ' '))
            // turn it sideways, so each stack is a row
            .transpose
            // get only the rows containing letters
            .slice(1 to 100 by 4)
            // remove spaces
            .map(_.filter(_ != ' '))
            .map(Stack(_: _*))

    protected def getMoves(movesInput: Seq[String]): Seq[Move] =
        movesInput.map(Move.apply)

object SupplyStacks extends Puzzle[String], SupplyStacksBase:
    override def solve(input: Seq[String]): String =
        val Seq(stacksInput, movesInput) = input.splitBy(_.isEmpty())
        val stacks                       = getStacks(stacksInput)
        val moves                        = getMoves(movesInput)

        moves.foreach(move => moveOneByOne(move, stacks))

        stacks.map(_.top).mkString

object SupplyStacks2 extends Puzzle[String], SupplyStacksBase:
    override def solve(input: Seq[String]): String =
        val Seq(stacksInput, movesInput) = input.splitBy(_.isEmpty())
        val stacks                       = getStacks(stacksInput)
        val moves                        = getMoves(movesInput)

        moves.foreach(move => moveAllAtOnce(move, stacks))

        stacks.map(_.top).mkString

case class Move(howMany: Int, from: Int, to: Int)

object Move:
    def apply(input: String): Move =
        // move 6 from 1 to 7
        val Array(howMany, from, to) = input.substring(5).split(" [a-z]+ ")
        // convert indexes to 0-based, so it's easier to use them later
        Move(howMany.toInt, from.toInt - 1, to.toInt - 1)

private def moveOneByOne(move: Move, stacks: Seq[Stack[Char]]): Unit =
    for i <- 1 to move.howMany do stacks(move.to).push(stacks(move.from).pop())

private def moveAllAtOnce(move: Move, stacks: Seq[Stack[Char]]): Unit =
    val temp = Stack[Char]()
    for i <- 1 to move.howMany do temp.push(stacks(move.from).pop())
    for i <- 1 to move.howMany do stacks(move.to).push(temp.pop())
