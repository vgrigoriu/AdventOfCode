

class SeqExtensionsSuite extends munit.FunSuite:
  test("can split list of numbers") {
    val input = List(1, 2, 3, 5, 7, 8, 9)

    val result = input.splitBy(_ % 2 == 0)
    assertEquals(result, Seq(Seq(1), Seq(3, 5, 7), Seq(9)))
  }
