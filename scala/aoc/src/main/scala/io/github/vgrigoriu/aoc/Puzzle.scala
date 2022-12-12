package io.github.vgrigoriu.aoc

import java.nio.file.Files.readAllLines
import java.nio.file.Paths
import scala.jdk.CollectionConverters.*
import java.nio.file.NoSuchFileException

trait Puzzle[+T]:
    def solve(input: Seq[String]): T

    def exampleResult: Option[T] = None

object PuzzleRunner:
    def run[T](puzzle: Puzzle[T]): Unit =
        given CanEqual[T, T] = CanEqual.derived

        // Try to run and verify the result for the example input.
        // Do nothing if the example input file doesn't exist
        // or if the actual object returns None as exampleResult.
        for
            exampleInput <- readExampleInput(puzzle)
            actualResult = puzzle.solve(exampleInput)
            expectedResult <- puzzle.exampleResult
        do
            if actualResult == expectedResult then
                println("Correct result for example input")
            else
                println(
                  s"Expected result for example input is $expectedResult but got $actualResult.",
                )

        println(puzzle.solve(readInput(puzzle)))
        
    private def readExampleInput(puzzle: Puzzle[_]): Option[Seq[String]] =
        try
            Some(
                readAllLines(
                    Paths.get(s"input/${name(puzzle)}-example.txt"),
                    ).asScala.toSeq,
                    )
                    catch case e: NoSuchFileException => None
                    
    private def readInput(puzzle: Puzzle[_]): Seq[String] =
        readAllLines(Paths.get(s"input/${name(puzzle)}.txt")).asScala.toSeq
                        
    private def name(puzzle: Puzzle[_]): String =
        puzzle.getClass.getSimpleName.replaceAll("\\$|\\d", "")
