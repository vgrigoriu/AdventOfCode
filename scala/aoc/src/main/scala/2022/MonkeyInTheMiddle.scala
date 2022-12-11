import scala.collection.mutable
object MonkeyInTheMiddle extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        getMonkeys(input).length

    override def exampleResult: Option[Int] = Some(10605)

case class MonkeyDefinition(
    private val operation: Int => Int,
    private val test: Int => Boolean,
    trueMonkey: Int,
    falseMonkey: Int,
)

class Monkey(
    val definition: MonkeyDefinition,
    val items: mutable.Queue[Int],
    var noItemsInspected: Int = 0,
)

private def getMonkeys(input: Seq[String]): Seq[Monkey] =
    input.splitBy(_.isEmpty).map(getMonkey)

private def getMonkey(input: Seq[String]): Monkey =
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    val items       = input(1).substring(18).split(", ").map(_.toInt)
    val operation   = getOperation(input(2).substring(23))
    val divisor     = input(3).substring(21).toInt
    val test        = (_: Int) % divisor == 0
    val trueMonkey  = input(4).substring(29).toInt
    val falseMonkey = input(5).substring(30).toInt

    Monkey(
      MonkeyDefinition(operation, _ % divisor == 0, trueMonkey, falseMonkey),
      mutable.Queue.from(items),
    )

private def getOperation(input: String): Int => Int =
    // * 19
    // * old
    // + 2
    val op: Int => Int => Int =
        if input(0) == '*' then a => b => a * b else a => b => a + b
    if input.substring(2) == "old" then old => op(old)(old)
    else op(input.substring(2).toInt)
