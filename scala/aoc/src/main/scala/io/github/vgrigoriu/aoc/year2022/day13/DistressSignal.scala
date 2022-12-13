package io.github.vgrigoriu.aoc.year2022.day13

import io.github.vgrigoriu.aoc.Puzzle
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
    var result = 0
    var intDone = false
    while !input.eof && !intDone do
        if input.current.isDigit then
            result = result * 10 + (input.current - '0')
            input.advance()
        else
            intDone = true
    
    result

object DistressSignal extends Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(13)
    override def solve(input: Seq[String]): Int =
        input.length
    
