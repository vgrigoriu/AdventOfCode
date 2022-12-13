package io.github.vgrigoriu.aoc.year2022.day13

import io.github.vgrigoriu.aoc.Puzzle
import io.github.vgrigoriu.aoc.*
import scala.collection.mutable.ArrayBuffer

class Input(val s: String, var pos: Int = 0):
    def current: Char = s(pos)
    def advance(): Unit =
        pos += 1
    def eof: Boolean = s.length <= pos

def parse(s: String): List[Any] =
    // [[],[3,5,1],[3,[7,0,[4,8]],[4,[4,7,2,4]]]]
    parseList(Input(s))

def parseList(input: Input): List[Any] =
    assert(input.current == '[')
    val result = ArrayBuffer[Any]()
    input.advance()
    var listDone = false
    while !input.eof && !listDone do
        input.current match
            case ']' =>
                listDone = true
                input.advance()
            case '[' =>
                result.append(parseList(input))
            case c if c.isDigit =>
                result.append(parseInt(input))
            case ',' => input.advance()

    result.toList

def parseInt(input: Input): Int =
    assert(input.current.isDigit)
    var result  = 0
    var intDone = false
    while !input.eof && !intDone do
        if input.current.isDigit then
            result = result * 10 + (input.current - '0')
            input.advance()
        else intDone = true

    result

sealed trait M
case object Marker extends M

given CanEqual[M, M]   = CanEqual.derived
given CanEqual[M, Any] = CanEqual.derived

val leftFirst  = -1
val same       = 0
val rightFirst = 1

object PacketOrdering extends Ordering[List[Any]]:
    override def compare(left: List[Any], right: List[Any]): Int =
        val tail = left
            .zipAll(right, Marker, Marker)
            .dropWhile((e1, e2) => compareItem(e1, e2) == same)
        tail match
            case List() => same
            case List((l, r), _*) => compareItem(l, r)
    private def compareItem(left: Any, right: Any): Int =
        (left, right) match
            case (l: Int, r: Int)             => l.compareTo(r)
            case (l: List[Any], r: List[Any]) => compare(l, r)
            case (l: Int, r: List[Any])       => compare(List(l), r)
            case (l: List[Any], r: Int)       => compare(l, List(r))
            case (Marker, _)                  => leftFirst
            case (_, Marker)                  => rightFirst

object DistressSignal extends Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(13)
    override def solve(input: Seq[String]): Int =
        input
            .splitBy(_.isEmpty)
            .map { case Seq(left, right) => (parse(left), parse(right)) }
            .zipWithIndex
            .filter { case ((left, right), _) =>
                PacketOrdering.compare(left, right) == leftFirst
            }
            .map((_, index) => index + 1)
            .sum

object DistressSignal2 extends Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(140)
    override def solve(input: Seq[String]): Int =
        val dividerPacket1 = parse("[[2]]")
        val dividerPacket2 = parse("[[6]]")
        val inputPackets = input
            .filter(_.nonEmpty)
            .map(parse)
        val sortedPackets =
            (inputPackets ++ Seq(dividerPacket1, dividerPacket2)).sorted(
              PacketOrdering,
            )
        val dividerIndex1 = sortedPackets.indexOf(dividerPacket1) + 1
        val dividerIndex2 = sortedPackets.indexOf(dividerPacket2) + 1

        dividerIndex1 * dividerIndex2
