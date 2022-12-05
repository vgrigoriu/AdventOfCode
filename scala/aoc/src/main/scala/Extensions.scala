import scala.collection.mutable.ArrayBuffer

extension [T](seq: Seq[T])
    def splitBy(pred: T => Boolean): Seq[Seq[T]] =
        val all     = ArrayBuffer[Seq[T]]()
        var current = ArrayBuffer[T]()
        for i <- seq do
            if pred(i) then
                all.addOne(current.toSeq)
                current = ArrayBuffer[T]()
            else current.addOne(i)

        all.addOne(current.toSeq)

        all.toSeq

    def slice(range: Range): Seq[T] =
        seq.zipWithIndex
            .filter { case (_, index) => range.contains(index) }
            .map { case (elem, _) => elem }

extension (range: Range)
    def fullyContains(otherRange: Range): Boolean =
        range.contains(otherRange.start) && range.contains(otherRange.end)

    def overlaps(otherRange: Range): Boolean =
        range.contains(otherRange.start) || range.contains(
          otherRange.end,
        ) || otherRange.contains(range.start)
