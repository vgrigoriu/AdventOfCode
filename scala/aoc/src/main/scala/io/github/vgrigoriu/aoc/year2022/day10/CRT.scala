package io.github.vgrigoriu.aoc.year2022.day10

import scala.collection.mutable.ArrayBuffer
import io.github.vgrigoriu.aoc.Puzzle

object CRT extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val instructions = input.flatMap(parse)
        var lastCycle    = Cycle(1, 1)
        val cycles       = ArrayBuffer(lastCycle)
        instructions.foreach(instruction =>
            lastCycle = instruction match
                case Noop()  => Cycle(lastCycle.no + 1, lastCycle.value)
                case Addx(v) => Cycle(lastCycle.no + 1, lastCycle.value + v)
            cycles.append(lastCycle),
        )
        cycles.filter(_.no % 40 == 20).map(c => c.no * c.value).sum

object CRT2 extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val instructions = input.flatMap(parse)
        var lastCycle    = Cycle(1, 1)
        val cycles       = ArrayBuffer(lastCycle)
        instructions.foreach(instruction =>
            lastCycle = instruction match
                case Noop()  => Cycle(lastCycle.no + 1, lastCycle.value)
                case Addx(v) => Cycle(lastCycle.no + 1, lastCycle.value + v)
            cycles.append(lastCycle),
        )

        val lines = cycles
            .map(c =>
                if math.abs((c.no - 1) % 40 - c.value) <= 1 then '#' else '.',
            )
            .grouped(40)
            .map(_.mkString)
            .toList
        lines.foreach(println)
        1

sealed abstract class Instruction
case class Noop()           extends Instruction
case class Addx(value: Int) extends Instruction

case class Cycle(no: Int, value: Int)

private def parse(s: String): Seq[Instruction] =
    // noop
    // addx 24
    s.substring(0, 4) match
        case "noop" => Seq(Noop())
        case "addx" => Seq(Noop(), Addx(s.substring(5).toInt))
