import java.nio.file.Files.readAllLines
import java.nio.file.Paths
import scala.jdk.CollectionConverters.*

trait Input:
    this: Puzzle =>
        def readInput(): Seq[String] =
            readAllLines(Paths.get(s"input/${this.name}.txt")).asScala.toSeq
