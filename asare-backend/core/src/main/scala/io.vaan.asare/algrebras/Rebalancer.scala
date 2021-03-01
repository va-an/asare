package io.vaan.asare.algrebras

import io.vaan.asare.domain.rebalance._
import cats._
import cats.syntax.all._

trait Rebalancer[F[_]] {
  def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio]
  def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio]
  def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio]
}

object Rebalancer {
  def make[F[_]: Applicative](): F[Rebalancer[F]] = // TODO: why F[A[F]] instead A[F]?
    F.pure { // TODO: pure?
      new Rebalancer[F] {
        override def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio] =
          F.pure {
            val sum = portfolio.values.sum
            portfolio.map {
              case (ticker: String, value: Double) => (ticker, value / sum.toDouble * 100)
            }
          }

        override def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio] =
          F.pure(
            rebalanceInput.expectedAllocation.map {
              case (ticker: String, value: Double) => (ticker, value / 100 * rebalanceInput.target)
            }
          )

        override def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio] =
          F.map(calcExpectedPortfolio(rebalanceInput)) { expectedPortfolio =>
            expectedPortfolio.map {
              case (ticker: String, value: Double) =>
                (ticker, value - rebalanceInput.actualPortfolio(ticker))
            }
          }
      }
    }
}
