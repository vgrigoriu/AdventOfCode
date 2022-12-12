import scala.collection.mutable

object HillClimbing extends Puzzle[Int]:
    type Heights = Map[Node, Int]

    override def exampleResult: Option[Int] = Some(31)

    override def solve(input: Seq[String]): Int =
        val Grid(height, width, heights, start, end) = parse(input)
        
        val distances: mutable.Map[Node, Int] = mutable.Map.from(heights.keys.map((_, Int.MaxValue)))
        distances(start) = 0
        val unvisited: mutable.Set[Node] = mutable.Set.from(heights.keys)
        
        var current = start
        while unvisited.contains(end) do
            val neighbors = getAccessibleNeighbors(heights, height, width, current)
            val unvisitedNeighbors = neighbors.filter(unvisited.contains)
            for neighbor <- unvisitedNeighbors do
                if distances(current) + 1 < distances(neighbor) then
                    distances(neighbor) = distances(current) + 1
            
            unvisited.remove(current)
            if unvisited.nonEmpty then
                current = unvisited.toList.sortBy(distances(_)).head
            // for
            //     row <- 0 until height
            //     col <- 0 until width
            // do
            //     val d = distances(Node(row, col))
            //     if d == Int.MaxValue then
            //         print("  ∞")
            //     else
            //         print(f"$d%3d")
            //     if col == width - 1 then println(s"    ${input(row)}")
            
            // println()

        distances(end)


    case class Grid(height: Int, width: Int, heights: Heights, start: Node, end: Node)

    private def parse(input: Seq[String]): Grid =
        val height = input.length
        // Assume all rows have the same length.
        val width = input.head.length
        val heights = mutable.Map[Node, Int]()
        var start = Node(-1, -1)
        var end = Node(-1, -1)
        for
            (s, row) <- input.zipWithIndex
            (c, col) <- s.zipWithIndex
        do
            if c == 'S' then
                start = Node(row, col)
                heights(start) = 'a' - 'a'
            else if c == 'E' then
                end = Node(row, col)
                heights(end) = 'z' - 'a'
            else
                heights(Node(row, col)) = c - 'a'
        
        val grid = Grid(height, width, heights.toMap, start, end)
        //println(grid)
        grid


    case class Node(row: Int, col: Int)
    given CanEqual[Node, Node] = CanEqual.derived

    private def getAccessibleNeighbors(
        heights: Heights,
        height: Int,
        width: Int,
        node: Node,
    ): Set[Node] =
        getAllNeighbors(node, height, width).filter(n => heights(n) <= heights(node) + 1)

    def getAllNeighbors(node: Node, height: Int, width: Int): Set[Node] =
        Set(
            Node(node.row - 1, node.col),
            Node(node.row + 1, node.col),
            Node(node.row, node.col - 1),
            Node(node.row, node.col + 1),
        ).filter(n => 0 <= n.row && n.row < height && 0 <= n.col && n.col < width)
