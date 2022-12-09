object RopeBridge extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val moves = input.flatMap(parse)
        val initialGrid = Grid(
            Seq(RopeEnd(0, 0), RopeEnd(0, 0)),
            Set()
        )
        val finalGrid = moves.foldLeft(initialGrid)((grid, move) => apply(grid, move))
        finalGrid.visited.size

    override def exampleResult: Option[Int] = Some(13)

object RopeBridge2 extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val moves = input.flatMap(parse)
        val initialGrid = Grid(
            Seq.fill(10)(RopeEnd(0, 0)),
            Set()
        )
        val finalGrid = moves.foldLeft(initialGrid)((grid, move) => apply(grid, move))
        finalGrid.visited.size

    override def exampleResult: Option[Int] = Some(1)

enum RopeMove:
    case Up
    case Down
    case Left
    case Right

given CanEqual[RopeMove, RopeMove] = CanEqual.derived

private def parse(s: String): Seq[RopeMove] =
    // R 4
    // U 4
    // L 3
    // D 1
    val steps = s.substring(2).toInt
    val direction = s(0) match
        case 'U' => RopeMove.Up
        case 'D' => RopeMove.Down
        case 'L' => RopeMove.Left
        case 'R' => RopeMove.Right
    
    Seq.fill(steps)(direction)

case class RopeEnd(x: Int, y: Int)

case class Grid(knots: Seq[RopeEnd], visited: Set[RopeEnd])

private def areTouching(head: RopeEnd, tail: RopeEnd): Boolean =
    math.abs(head.x - tail.x) <= 1 && math.abs(head.y - tail.y) <= 1

private def adjustTail2(head: RopeEnd, tail: RopeEnd): RopeEnd =
    // There are 8 positions where the tail could be relative to the head:
    // T   T   T
    // T   H   T
    // T   T   T
    if tail.x < head.x then
        if tail.y < head.y then
            // SW
            tail.copy(tail.x + 1, tail.y + 1)
        else if tail.y == head.y then
            // W
            tail.copy(x = tail.x + 1)
        else // tail.y > head.y
            // NW
            tail.copy(tail.x + 1, tail.y - 1)
    else if tail.x == head.x then
        if tail.y < head.y then
            // S
            tail.copy(y = tail.y + 1)
        else // tail.y > head.y
            // N
            tail.copy(y = tail.y - 1)
    else // tail.x > head.x
        if tail.y < head.y then
            // SE
            tail.copy(tail.x - 1, tail.y + 1)
        else if tail.y == head.y then
            // E
            tail.copy(x = tail.x - 1)
        else // tail.y > head.y
            // NE
            tail.copy(tail.x - 1, tail.y - 1)

private def adjustTail(head: RopeEnd, tail: RopeEnd): RopeEnd =
    if areTouching(head, tail) then
        tail
    else
        adjustTail2(head, tail)

private def apply(grid: Grid, move: RopeMove): Grid =
    val newHead = move match
        case RopeMove.Up => grid.knots(0).copy(y = grid.knots(0).y + 1)
        case RopeMove.Down => grid.knots(0).copy(y = grid.knots(0).y - 1)
        case RopeMove.Left => grid.knots(0).copy(x = grid.knots(0).x - 1)
        case RopeMove.Right => grid.knots(0).copy(x = grid.knots(0).x + 1)
    
    val newKnots = grid.knots.tail.foldLeft(Seq(newHead))((movedHeads, currentHead) =>
        movedHeads :+ adjustTail(movedHeads.last, currentHead)
    )
    Grid(newKnots, grid.visited + newKnots.last)
