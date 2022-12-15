package io.github.vgrigoriu.aoc.year2022.day15

import scala.collection.mutable
import io.github.vgrigoriu.aoc.Puzzle

private case class Position(x: Int, y: Int)
private case class Pair(sensor: Position, beacon: Position)

private def parsePosition(s: String): Position =
    // x=2, y=18
    val Array(xStr, yStr) = s.split(", ")
    // xStr: "x=2", yStr: "y=18"
    Position(xStr.substring(2).toInt, yStr.substring(2).toInt)

private def parse(s: String): Pair =
    // Sensor at x=2, y=18: closest beacon is at x=-2, y=15
    val Array(sensorStr, beaconStr) = s.split(": ")
    // sensorStr: "Sensor at x=2, y=18"
    val sensor = parsePosition(sensorStr.substring(10))
    // beaconStr: "closest beacon is at x=-2, y=15"
    val beacon = parsePosition(beaconStr.substring(21))

    Pair(sensor, beacon)

private def getTargetRow(sensorCount: Int): Int =
    // Target row is not part of the input, so hardcode it here
    // based on the number of sensors in the example and real inputs
    if sensorCount == 14 then 10 else 2000000

private def getMaxCoord(sensorCount: Int): Int =
    // MaxCoord is not part of the input, so hardcode it here
    // based on the number of sensors in the example and real inputs
    if sensorCount == 14 then 20 else 4000000

private def distance(p1: Position, p2: Position) =
    math.abs(p1.x - p2.x) + math.abs(p1.y - p2.y)

private def distance(pair: Pair): Int =
    distance(pair.sensor, pair.beacon)

private def getExcludedToOneSide(p: Position, targetY: Int, dist: Int): Int =
    // Suppose dist is 6:
    // ...p...
    // ...|...
    // ...|...
    // ...|...
    // .-----.
    dist - math.abs(p.y - targetY)

object BeaconExclusionZone extends Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(26)

    override def solve(input: Seq[String]): Int =
        val pairs   = input.map(parse)
        val targetY = getTargetRow(pairs.length)

        val exclusions: mutable.Set[Int] = mutable.Set()
        for pair <- pairs do
            val dist = distance(pair)
            val excludedToOneSide =
                getExcludedToOneSide(pair.sensor, targetY, dist)
            if 0 <= excludedToOneSide then
                exclusions.addAll(
                  pair.sensor.x - excludedToOneSide to pair.sensor.x + excludedToOneSide,
                )

        val beaconsInExclusionZone = pairs
            .filter { case Pair(_, beacon) =>
                beacon.y == targetY && exclusions.contains(beacon.x)
            }
            .map { case Pair(_, beacon) => beacon }
            .toSet
        exclusions.size - beaconsInExclusionZone.size

private def isTooClose(p: Position, pair: Pair): Boolean =
    //println(s"Checking $p and $pair: d1 = ${distance(p, pair.sensor)}, d2 = ${distance(pair)}: ${distance(p, pair.sensor) <= distance(pair)}")
    distance(p, pair.sensor) <= distance(pair)

private def isTooClose(p: Position, pairs: Seq[Pair]): Boolean =
    pairs.view.exists(pair => isTooClose(p, pair))

object BeaconExclusionZone2 extends Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(56000011)

    override def solve(input: Seq[String]): Int =
        val pairs   = input.map(parse)
        val maxCoord = getMaxCoord(pairs.size)

        val allBeacons = pairs.map(p => p.beacon).toSet

        val exclusions: mutable.Set[Position] = mutable.Set()
        for x <- 0 to maxCoord
            y <- 0 to maxCoord
        do
            if !isTooClose(Position(x, y), pairs) && !allBeacons.contains(Position(x, y)) then
                println(x * 4000000 + y)
        
        exclusions.size