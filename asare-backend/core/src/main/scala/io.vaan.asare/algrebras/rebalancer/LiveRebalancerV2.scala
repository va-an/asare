package io.vaan.asare.algrebras.rebalancer

import cats.Functor
import cats.syntax.all._
import cats.effect.Sync
import io.vaan.asare.domain.rebalance
import io.vaan.asare.utils.NumUtils._

class LiveRebalancerV2[F[_]: Functor: Sync] private[rebalancer] () extends LiveRebalancerV1[F] {
  override def calcCurrentAllocation(portfolio: rebalance.Portfolio): F[rebalance.Portfolio] =
    super
      .calcCurrentAllocation(portfolio)
      .map(portfolio =>
        portfolio.map {
          case (ticker: String, value: Double) => (ticker, value.roundTo2)
        }
      )
}
