package eu.vgrigoriu.adventofcode._2015_01

class FloorCounter(val up: Char, val down: Char) {
  def countFloors(directions: String): Int = {
    directions.map(x).sum
  }

  def firstNegativeFloor(directions: String): Int = {
    val floors = directions.scanLeft(0)(_ + x(_))

    floors.zipWithIndex.filter(x => x._1 < 0).head._2
  }

  private def x(direction: Char): Int = {
    if (direction == up)
      1
    else if (direction == down)
      -1
    else
      0
  }
}
