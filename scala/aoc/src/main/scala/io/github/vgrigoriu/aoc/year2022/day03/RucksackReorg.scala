package io.github.vgrigoriu.aoc.year2022.day03

import io.github.vgrigoriu.aoc.Puzzle

object RucksackReorg extends Puzzle[Int]:
    def solve(input: Seq[String]): Int =
        val rucksacksWithCompartments = input.map { rucksack =>
            val compartmentLength = rucksack.length / 2
            (
              rucksack.substring(0, compartmentLength),
              rucksack.substring(compartmentLength),
            )
        }
        val rucksacksWithCompartmentsAsSets = rucksacksWithCompartments.map {
            case (c1, c2) => (c1.toSet, c2.toSet)
        }
        val commonElements = rucksacksWithCompartmentsAsSets.map {
            case (c1, c2) =>
                (c1 intersect c2).head
        }
        val priorities = commonElements.map(priority)

        priorities.sum

object RucksackReorg2 extends Puzzle[Int]:
    def solve(input: Seq[String]): Int =
        val rucksacksAsSets = input.map(_.toSet)
        val elfGroups       = rucksacksAsSets.grouped(3)
        val badges = elfGroups.map(elfGroup =>
            elfGroup.reduce((e1, e2) => e1 intersect e2).head,
        )
        val priorities = badges.map(priority)

        priorities.sum

private def priority(c: Char): Int =
    if 'a' <= c && c <= 'z' then c - 'a' + 1
    else if 'A' <= c && c <= 'Z' then c - 'A' + 27
    else 0
