package io.vaan.asare.algrebras

import io.vaan.asare.domain.rebalance._

trait Rebalancer[F[_]] {
  def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio]
  def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio]
  def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio]
}

object Rebalancer {
  def make[F[_]](): Rebalancer[F] =
    new Rebalancer[F] {
      override def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio] =
        ???

      override def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio] =
        ???

      override def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio] =
        ???
    }
}
