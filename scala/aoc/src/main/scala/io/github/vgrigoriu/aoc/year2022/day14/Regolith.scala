package io.github.vgrigoriu.aoc.year2022.day14

import io.github.vgrigoriu.aoc.Puzzle

case class Coord(x: Int, y: Int)
case class Path(coords: Seq[Coord])
case class Cave(paths: Seq[Path], sandSource: Coord)

private def parsePath(s: String): Path =
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    val coords = s
        .split(" -> ")
        .map(_.split(","))
        .map { case Array(x, y) => Coord(x.toInt, y.toInt) }
        .toSeq

    Path(coords)

private def parse(input: Seq[String]): Cave =
    // 498,4 -> 498,6 -> 496,6
    // 503,4 -> 502,4 -> 502,9 -> 494,9
    Cave(input.map(parsePath), Coord(500, 0))

private def limits(cave: Cave): (Coord, Coord) =
    val allCoords = cave.paths.flatMap(_.coords)
    val allXs     = allCoords.map(_.x) :+ cave.sandSource.x
    val allYs     = allCoords.map(_.y) :+ cave.sandSource.y

    (Coord(allXs.min, allYs.min), Coord(allXs.max, allYs.max))

private def translate(c: Coord, topLeft: Coord): Coord =
    Coord(c.x - topLeft.x, c.y - topLeft.y)

private def translate(path: Path, topLeft: Coord): Path =
    Path(path.coords.map(c => translate(c, topLeft)))

private def translate(cave: Cave, topLeft: Coord): Cave =
    Cave(cave.paths.map(translate(_, topLeft)), translate(cave.sandSource, topLeft))

private def draw(c1: Coord, c2: Coord, caveMap: Array[Array[Char]]): Unit =
    if c1.x == c2.x then
        // draw vertical line
        val step = if c1.y < c2.y then 1 else -1
        (c1.y to c2.y by step).foreach(y => caveMap(y)(c1.x) = '#')
    else if c1.y == c2.y then
        // draw horizontal line
        val step = if c1.x < c2.x then 1 else -1
        (c1.x to c2.x by step).foreach(x => caveMap(c1.y)(x) = '#')

private def draw(path: Path, caveMap: Array[Array[Char]]): Unit =
    path.coords.sliding(2).foreach { case Seq(c1, c2) => draw(c1, c2, caveMap) }

private def buildCaveMap(cave: Cave): Array[Array[Char]] =
    val (_, bottomRight) = limits(cave)
    //val result = Array.ofDim[Char](bottomRight.x + 1, bottomRight.y + 1)
    val result = Array.fill(bottomRight.y + 1)(Array.fill(bottomRight.x + 1)('.'))
    result(cave.sandSource.y)(cave.sandSource.x) = '+'
    cave.paths.foreach(path => draw(path, result))

    result

private def printCaveMap(caveMap: Array[Array[Char]]): Unit =
    for row <- caveMap do
        for c <- row do print(c)
        println

object Regolith extends Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(24)

    override def solve(input: Seq[String]): Int =
        val originalCave           = parse(input)
        val (topLeft, bottomRight) = limits(originalCave)
        val cave                   = translate(originalCave, topLeft)
        val caveMap                = buildCaveMap(cave)
        printCaveMap(caveMap)

        cave.paths.length
