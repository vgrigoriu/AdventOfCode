package eu.vgrigoriu.adventofcode._2015_01

import org.scalatest._

class NotQuiteLispSpec extends FunSuite with DiagrammedAssertions {
  test("Math works") {
    assert(1 + 1 == 2)
  }

  test("FloorCounter remembers up") {
    val fc = new FloorCounter('(', ')')
    assert(fc.up == '(')
  }

  test("FloorCounter counts empty directions") {
    val fc = new FloorCounter('(', ')')
    assert(fc.countFloors("") == 0)
  }

  test("FloorCounter counts going up") {
    val fc = new FloorCounter('(', ')')
    assert(fc.countFloors("(((") == 3)
  }

  test("FloorCounter counts up and down") {
    val fc = new FloorCounter('(', ')')
    assert(fc.countFloors("()()") == 0)
    assert(fc.countFloors(")())())") == -3)
  }
}

