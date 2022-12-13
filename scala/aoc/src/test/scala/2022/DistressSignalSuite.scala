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

    test("Can compare two empty lists") {
        val result = PacketOrdering.compare(List(), List())
        assertEquals(result, same)
    }

    test("Can compare one list shorter") {
        val result = PacketOrdering.compare(List(1, 2), List(1, 2, 3))
        assertEquals(result, leftFirst)
    }

    test("Can compare other list shorter") {
        val result = PacketOrdering.compare(List(1, 2, 3, 4), List(1, 2, 3))
        assertEquals(result, rightFirst)
    }

    test("Int is promoted to List") {
        val result = PacketOrdering.compare(List(List(1)), List(1))
        assertEquals(result, same)
    }

    test("Compare example input 1") {
        val result = PacketOrdering.compare(parse("[1,1,3,1,1]"), parse("[1,1,5,1,1]"))
        assertEquals(result, leftFirst)
    }

    test("Compare example input 2") {
        val result = PacketOrdering.compare(parse("[[1],[2,3,4]]"), parse("[[1],4]"))
        assertEquals(result, leftFirst)
    }

    test("Compare example input 3") {
        val result = PacketOrdering.compare(parse("[9]"), parse("[[8,7,6]]"))
        assertEquals(result, rightFirst)
    }

    test("Compare example input 4") {
        val result = PacketOrdering.compare(parse("[[4,4],4,4]"), parse("[[4,4],4,4,4]"))
        assertEquals(result, leftFirst)
    }

    test("Compare example input 5") {
        val result = PacketOrdering.compare(parse("[7,7,7,7]"), parse("[7,7,7]"))
        assertEquals(result, rightFirst)
    }

    test("Compare example input 6") {
        val result = PacketOrdering.compare(parse("[]"), parse("[3]"))
        assertEquals(result, leftFirst)
    }
