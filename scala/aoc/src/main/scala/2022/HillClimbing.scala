import scala.collection.mutable
trait HillClimbingBase:
    case class Node(row: Int, col: Int)
    given CanEqual[Node, Node] = CanEqual.derived

    type Heights = Map[Node, Int]

    case class Grid(height: Int, width: Int, heights: Heights, start: Node, end: Node)

    protected def getMinDistance(grid: Grid): Int =
        val Grid(height, width, heights, start, end) = grid
        
        val distances: mutable.Map[Node, Int] = mutable.Map.from(heights.keys.map((_, Int.MaxValue)))
        distances(start) = 0
        val unvisited: mutable.Set[Node] = mutable.Set.from(heights.keys)
        
        var current = start
        while unvisited.contains(end) do
            val neighbors = getAccessibleNeighborsBackwards(heights, height, width, current)
            val unvisitedNeighbors = neighbors.filter(unvisited.contains)
            for neighbor <- unvisitedNeighbors do
                if distances(current) + 1 < distances(neighbor) then
                    distances(neighbor) = distances(current) + 1
            
            unvisited.remove(current)
            if unvisited.nonEmpty then
                current = unvisited.toList.sortBy(distances(_)).head

        distances(end)
    
    private def getAccessibleNeighbors(
        heights: Heights,
        height: Int,
        width: Int,
        node: Node,
    ): Set[Node] =
        getAllNeighbors(node, height, width).filter(n => heights(n) <= heights(node) + 1)

    protected def getAccessibleNeighborsBackwards(
        heights: Heights,
        height: Int,
        width: Int,
        node: Node,
    ): Set[Node] =
        getAllNeighbors(node, height, width).filter(n => heights(node) <= heights(n) + 1)

    def getAllNeighbors(node: Node, height: Int, width: Int): Set[Node] =
        Set(
            Node(node.row - 1, node.col),
            Node(node.row + 1, node.col),
            Node(node.row, node.col - 1),
            Node(node.row, node.col + 1),
        ).filter(n => 0 <= n.row && n.row < height && 0 <= n.col && n.col < width)

    protected def parse(input: Seq[String]): Grid =
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
        grid

object HillClimbing extends HillClimbingBase, Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(31)

    override def solve(input: Seq[String]): Int =
        val grid = parse(input)
        getMinDistance(grid.copy(start = grid.end, end = grid.start))

object HillClimbing2 extends HillClimbingBase, Puzzle[Int]:
    override def exampleResult: Option[Int] = Some(29)

    override def solve(input: Seq[String]): Int =
        val grid = parse(input)
        getMinDistanceToGround(grid.copy(start = grid.end))
    
    protected def getMinDistanceToGround(grid: Grid): Int =
        val Grid(height, width, heights, start, _) = grid
        
        val distances: mutable.Map[Node, Int] = mutable.Map.from(heights.keys.map((_, Int.MaxValue)))
        distances(start) = 0
        val unvisited: mutable.Set[Node] = mutable.Set.from(heights.keys)
        
        var current = start
        var foundGround = false
        while !foundGround do
            val neighbors = getAccessibleNeighborsBackwards(heights, height, width, current)
            val unvisitedNeighbors = neighbors.filter(unvisited.contains)
            for neighbor <- unvisitedNeighbors do
                if distances(current) + 1 < distances(neighbor) then
                    distances(neighbor) = distances(current) + 1
                    if heights(neighbor) == 0 then
                        foundGround = true
            
            unvisited.remove(current)
            if unvisited.nonEmpty then
                current = unvisited.toList.sortBy(distances(_)).head

        distances.filter((node, d) => heights(node) == 0).map((node, d) => d).min
        