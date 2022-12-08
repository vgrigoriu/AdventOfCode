object TreeHouse extends Puzzle[Int]:
    override def solve(input: Seq[String]): Int =
        val forest = toDigitArray(input)
        countVisibleTrees(forest)

    override def exampleResult: Option[Int] = Some(21)

object TreeHouse2 extends Puzzle[Int]:
    def solve(input: Seq[String]): Int =
        val forest = toDigitArray(input)
        maxScenicScore(forest)

private def toDigitArray(input: Seq[String]): Vector[Vector[Int]] =
    input.map(row => row.map(_ - '0').toVector).toVector

private def countVisibleTrees(forest: Vector[Vector[Int]]): Int =
    val visibleTrees =
        for
            row <- 0 until forest.length
            col <- 0 until forest(row).length
            if isVisible(forest, row, col)
        yield 1

    visibleTrees.length

private def isVisible(
    forest: Vector[Vector[Int]],
    row: Int,
    col: Int,
): Boolean =
    val treeHeight      = forest(row)(col)
    val visibleFromWest = (0 until col).forall(c => forest(row)(c) < treeHeight)
    val visibleFromEast = (col + 1 until forest(row).length).forall(c =>
        forest(row)(c) < treeHeight,
    )
    val visibleFromNorth =
        (0 until row).forall(r => forest(r)(col) < treeHeight)
    val visibleFromSouth =
        (row + 1 until forest.length).forall(r => forest(r)(col) < treeHeight)

    visibleFromWest || visibleFromEast || visibleFromNorth || visibleFromSouth

private def maxScenicScore(forest: Vector[Vector[Int]]): Int =
    val allScores =
        for
            row <- 0 until forest.length
            col <- 0 until forest(row).length
        yield scenicScore(forest, row, col)
    allScores.max

private def scenicScore(forest: Vector[Vector[Int]], row: Int, col: Int): Int =
    val treeHeight = forest(row)(col)

    val blocksViewWest =
        (0 until col).reverse.find(c => treeHeight <= forest(row)(c))
    val scoreWest = blocksViewWest match
        case None    => col
        case Some(c) => col - c

    val blocksViewEast = (col + 1 until forest(row).length).find(c =>
        treeHeight <= forest(row)(c),
    )
    val scoreEast = blocksViewEast match
        case None    => forest(row).length - 1 - col
        case Some(c) => c - col

    val blocksViewNorth =
        (0 until row).reverse.find(r => treeHeight <= forest(r)(col))
    val scoreNorth = blocksViewNorth match
        case None    => row
        case Some(r) => row - r

    val blocksViewSouth =
        (row + 1 until forest.length).find(r => treeHeight <= forest(r)(col))
    val scoreSouth = blocksViewSouth match
        case None    => forest.length - 1 - row
        case Some(r) => r - row

    scoreWest * scoreEast * scoreNorth * scoreSouth
