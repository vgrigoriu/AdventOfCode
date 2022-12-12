import HillClimbing.*

class HillClimbingSuite extends munit.FunSuite:
    test("Can get all neighbors in NW corner") {
        val result = getAllNeighbors(Node(0, 0), 3, 3)
        assertEquals(
          result,
          Set(
            Node(1, 0),
            Node(0, 1),
          ),
        )
    }

    test("Can get all neighbors in center") {
        val result = getAllNeighbors(Node(1, 1), 3, 3)
        assertEquals(
          result,
          Set(
            Node(0, 1),
            Node(1, 0),
            Node(1, 2),
            Node(2, 1),
          ),
        )
    }

    test("Can get all neighbors in SE corner") {
        val result = getAllNeighbors(Node(2, 2), 3, 3)
        assertEquals(
          result,
          Set(
            Node(1, 2),
            Node(2, 1),
          ),
        )
    }
