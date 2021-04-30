package io.vaan.asare.bot.algebras

import cats.effect.Sync
import io.vaan.asare.bot.domain.rebalance._
import cats.MonadError
import cats.MonadThrow
import cats.ApplicativeError

trait InputParser[F[_]] {
  def parse(input: String): F[RebalanceInput]
}

object LiveInputParser {
  def make[F[_]: Sync]: F[InputParser[F]] =
    Sync[F].delay(
      new InputParser[F] {
        override def parse(input: String): F[RebalanceInput] =
          Sync[F]
            .delay {
              val inputList =
                input.linesIterator
                  .map(line => line.split(" "))
                  .toList

              val currentPortfolio = inputList.map { array =>
                val value = array(1)
                  .filterNot(x => x == '_')
                  .toDouble

                array(0) -> value
              }.toMap

              val requiredAllocation = inputList
                .map(array => array(0) -> array(2).toDouble)
                .toMap

              RebalanceInput(
                currentPortfolio = currentPortfolio,
                requiredAllocation = requiredAllocation,
                target = None
              )
            }
      }
    )
}
