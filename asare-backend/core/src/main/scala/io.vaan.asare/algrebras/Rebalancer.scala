package io.vaan.asare.algrebras

import io.vaan.asare.domain.rebalance._
import cats.Applicative

trait Rebalancer[F[_]] {
  def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio]
  def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio]
  def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio]
}

object Rebalancer {
  def make[F[_]: Applicative](): Rebalancer[F] =
    new Rebalancer[F] {
      override def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio] =
        F.pure(???)

      override def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio] =
        F.pure(???)

      override def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio] =
        F.pure(???)
    }
}
