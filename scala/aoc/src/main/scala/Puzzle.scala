import java.nio.file.Files.readAllLines
import java.nio.file.Paths
import scala.jdk.CollectionConverters.*

trait Puzzle:
  def solve(): Int

  def name: String = this.getClass.getSimpleName.replaceAll("\\$|\\d", "")

  protected def readInput(): Seq[String] =
    readAllLines(Paths.get(s"input/${this.name}.txt")).asScala.toSeq
