package io.vaan.asare.domain

object rebalance {
  type Portfolio = Map[String, Double]

  final case class RebalanceInput(
      currentPortfolio: Map[String, Double],
      requiredAllocation: Map[String, Double],
      target: Option[Double]
  )
}
