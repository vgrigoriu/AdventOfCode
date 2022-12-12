package io.github.vgrigoriu.aoc.year2022.day11

import scala.collection.mutable
import io.github.vgrigoriu.aoc.Puzzle
import io.github.vgrigoriu.aoc.*

object MonkeyInTheMiddle extends Puzzle[Long]:
    override def solve(input: Seq[String]): Long =
        val monkeys = getMonkeys(input)
        for round <- 1 to 20 do doRound(monkeys)

        val Seq(m1, m2, _*) = monkeys.sortBy(-_.noItemsInspected): @unchecked

        m1.noItemsInspected * m2.noItemsInspected

    override def exampleResult: Option[Long] = Some(10605L)

object MonkeyInTheMiddle2 extends Puzzle[Long]:
    override def solve(input: Seq[String]): Long =
        val monkeys = getMonkeys(input)
        for round <- 1 to 10000 do doRound(monkeys, relief = false)

        val Seq(m1, m2, _*) = monkeys.sortBy(-_.noItemsInspected): @unchecked

        m1.noItemsInspected * m2.noItemsInspected

    override def exampleResult: Option[Long] = Some(2713310158L)

case class MonkeyDefinition(
    operation: BigInt => BigInt,
    divisor: BigInt,
    trueMonkey: Int,
    falseMonkey: Int,
)

class Monkey(
    val definition: MonkeyDefinition,
    val items: mutable.Queue[BigInt],
    var noItemsInspected: Long = 0,
):
    override def toString(): String =
        s"inspected: $noItemsInspected; items: $items"

def doRound(monkeys: Seq[Monkey], relief: Boolean = true): Unit =
    monkeys.foreach(m => doTurn(monkeys, m, relief))

def doTurn(
    monkeys: Seq[Monkey],
    currentMonkey: Monkey,
    relief: Boolean = true,
): Unit =
    val m     = currentMonkey.definition
    val items = currentMonkey.items
    currentMonkey.noItemsInspected += items.length
    val modulo = monkeys.map(_.definition.divisor).reduce(_ * _)
    while items.nonEmpty do
        val item    = items.dequeue()
        val newItem = m.operation(item) / (if relief then 3 else 1) % modulo
        if newItem % m.divisor == 0 then
            monkeys(m.trueMonkey).items.enqueue(newItem)
        else monkeys(m.falseMonkey).items.enqueue(newItem)

private def getMonkeys(input: Seq[String]): Seq[Monkey] =
    input.splitBy(_.isEmpty).map(getMonkey)

private def getMonkey(input: Seq[String]): Monkey =
    // Monkey 0:
    //   Starting items: 79, 98
    //   Operation: new = old * 19
    //   Test: divisible by 23
    //     If true: throw to monkey 2
    //     If false: throw to monkey 3
    val items       = input(1).substring(18).split(", ").map(BigInt.apply)
    val operation   = getOperation(input(2).substring(23))
    val divisor     = input(3).substring(21).toInt
    val test        = (_: Int) % divisor == 0
    val trueMonkey  = input(4).substring(29).toInt
    val falseMonkey = input(5).substring(30).toInt

    Monkey(
      MonkeyDefinition(operation, BigInt(divisor), trueMonkey, falseMonkey),
      mutable.Queue.from(items),
    )

private def getOperation(input: String): BigInt => BigInt =
    // * 19
    // * old
    // + 2
    val op: BigInt => BigInt => BigInt =
        if input(0) == '*' then a => b => a * b else a => b => a + b
    if input.substring(2) == "old" then old => op(old)(old)
    else op(input.substring(2).toInt)
