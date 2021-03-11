package io.vaan.asare.algrebras.rebalancer

import cats.Functor
import cats.syntax.all._
import cats.effect.Sync
import io.vaan.asare.domain.rebalance._
import io.vaan.asare.utils.NumUtils._

class LiveRebalancerV2[F[_]: Functor: Sync] private[rebalancer] () extends LiveRebalancerV1[F] {
  override def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio] =
    super
      .calcCurrentAllocation(portfolio)
      .map(round)

  override def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio] =
    super
      .calcPurchase(rebalanceInput)
      .map(round)

  private def round(portfolio: Portfolio): Portfolio =
    portfolio.map {
      case (ticker: String, value: Double) => (ticker, value.roundTo2)
    }
}
