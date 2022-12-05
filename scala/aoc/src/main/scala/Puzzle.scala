import java.nio.file.Files.readAllLines
import java.nio.file.Paths
import scala.jdk.CollectionConverters.*

trait Puzzle[T]:
    def solve(): T

    def name: String = this.getClass.getSimpleName.replaceAll("\\$|\\d", "")

    protected def readInput(): Seq[String] =
        readAllLines(Paths.get(s"input/${this.name}.txt")).asScala.toSeq

trait IntPuzzle    extends Puzzle[Int]
trait StringPuzzle extends Puzzle[String]
