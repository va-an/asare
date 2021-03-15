package io.vaan.asare.algrebras.rebalancer

import io.vaan.asare.algrebras.rebalancer.RebalancerA

trait RebalancerV[F[_]] {
  def v1: RebalancerA[F]
  def v2: RebalancerA[F]
}
