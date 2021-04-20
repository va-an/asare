package io.vaan.asare.algrebras.rebalancer

import io.vaan.asare.domain.rebalance._
import cats._
import cats.effect.Sync
import cats.syntax.all._

trait RebalancerA[F[_]] {
  def calcCurrentAllocation(portfolio: Portfolio): F[Portfolio]
  def calcExpectedPortfolio(rebalanceInput: RebalanceInput): F[Portfolio]
  def calcPurchase(rebalanceInput: RebalanceInput): F[Portfolio]
}

object RebalancerA {
  def makeV[F[_]: Sync](): F[RebalancerV[F]] =
    F.delay {
      new RebalancerV[F] {
        def v1: RebalancerA[F] = new LiveRebalancerV1[F]
        def v2: RebalancerA[F] = new LiveRebalancerV2[F]
      }
    }
}
