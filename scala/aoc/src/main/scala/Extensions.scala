import scala.collection.mutable.ArrayBuffer

extension [T](list: Seq[T])
    def splitBy(pred: T => Boolean): Seq[Seq[T]] =
        val all     = ArrayBuffer[Seq[T]]()
        var current = ArrayBuffer[T]()
        for i <- list do
            if pred(i) then
                all.addOne(current.toSeq)
                current = ArrayBuffer[T]()
            else current.addOne(i)

        all.addOne(current.toSeq)

        all.toSeq

extension (range: Range)
    def fullyContains(otherRange: Range): Boolean =
        range.contains(otherRange.start) && range.contains(otherRange.end)

    def overlaps(otherRange: Range): Boolean =
        range.contains(otherRange.start) || range.contains(
          otherRange.end,
        ) || otherRange.contains(range.start)
