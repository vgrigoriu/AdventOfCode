package io.github.vgrigoriu.aoc

import scala.collection.mutable.ArrayBuffer

extension [T](seq: Seq[T])
    def splitBy(pred: T => Boolean): Seq[Seq[T]] =
        val all     = ArrayBuffer[Seq[T]]()
        var current = ArrayBuffer[T]()
        for elem <- seq do
            if pred(elem) then
                all.append(current.toSeq)
                current = ArrayBuffer[T]()
            else current.append(elem)

        all.append(current.toSeq)

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
