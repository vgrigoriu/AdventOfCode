package io.github.vgrigoriu.aoc.year2015.day16

import io.github.vgrigoriu.aoc.Puzzle

object AuntSue extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val aunts         = input.map(Aunt.apply)
        val matchingAunts = aunts.filter(_.matches(targetReading))
        matchingAunts(0).no

object AuntSue2 extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val aunts         = input.map(Aunt.apply)
        val matchingAunts = aunts.filter(_.matches2(targetReading))
        matchingAunts(0).no

val targetReading = Aunt(
  Map(
    "children"    -> 3,
    "cats"        -> 7,
    "samoyeds"    -> 2,
    "pomeranians" -> 3,
    "akitas"      -> 0,
    "vizslas"     -> 0,
    "goldfish"    -> 5,
    "trees"       -> 3,
    "cars"        -> 2,
    "perfumes"    -> 1,
  ),
  0,
)
case class Aunt(things: Map[String, Int], no: Int):
    def matches(target: Aunt): Boolean =
        things.keySet.forall(thing => things(thing) == target.things(thing))

    def matches2(target: Aunt): Boolean =
        things.keySet.forall(thing =>
            thing match {
                // the cats and trees readings indicates that there are greater than that many
                // the pomeranians and goldfish readings indicate that there are fewer than that many
                case "cats" | "trees" => things(thing) > target.things(thing)
                case "pomeranians" | "goldfish" =>
                    things(thing) < target.things(thing)
                case _ => things(thing) == target.things(thing)
            },
        )

object Aunt:
    def apply(input: String): Aunt =
        // Sue 1: cars: 9, akitas: 3, goldfish: 0
        val Array(auntNo, auntThings) = input.substring(4).split(": ", 2)
        val things = auntThings
            .split(", ")
            .map(_.split(": "))
            .map(parts => (parts(0), parts(1).toInt))
            .toMap

        Aunt(things, auntNo.toInt)
