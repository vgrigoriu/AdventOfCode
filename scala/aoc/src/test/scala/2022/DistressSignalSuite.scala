import io.github.vgrigoriu.aoc.year2022.day13.*

class DistressSignalSuite extends munit.FunSuite:
    test("Can parse an Int") {
        val result = parseInt(Input("123"))
        assertEquals(result, 123)
    }

    test("Can parse an empty List") {
        val result = parseList(Input("[]"))
        assertEquals(result, List())
    }

    test("Can parse a List with one Int") {
        val result = parseList(Input("[7]"))
        assertEquals(result, List(7))
    }

    test("Can parse a List with Ints and Lists") {
        val result = parseList(Input("[1,[2],3,4,[5,6]]"))
        assertEquals(result, List(1,List(2), 3, 4, List(5, 6)))
    }

    test("Can parse a nested List") {
        val result = parseList(Input("[1,[2,[3]],4,5,[6,7],[[[]]]]"))
        assertEquals(result, List(1,List(2, List(3)), 4, 5, List(6, 7), List(List(List()))))
    }
