package io.github.vgrigoriu.aoc.year2022.day01

import io.github.vgrigoriu.aoc.Puzzle
import io.github.vgrigoriu.aoc.*

trait CalorieCountingBase:
    protected class Calories(valuesAsStrings: Seq[String]):
        val values = valuesAsStrings.map(_.toInt)
        val total  = values.sum

    protected def getCaloriesPerElf(input: Seq[String]): Seq[Calories] =
        val groups = input.splitBy(_.isEmpty)

        groups.map(Calories(_))

object CalorieCounting extends CalorieCountingBase, Puzzle[Int]:
    def solve(input: Seq[String]): Int =
        getCaloriesPerElf(input).map(_.total).max

object CalorieCounting2 extends CalorieCountingBase, Puzzle[Int]:
    def solve(input: Seq[String]): Int =
        getCaloriesPerElf(input).map(_.total).sorted.reverse.take(3).sum
