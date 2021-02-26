package io.vaan.asare.domain

object rebalance {
  type Portfolio = Map[String, Double]

  final case class RebalanceInput(
      actualPortfolio: Map[String, Double],
      expectedAllocation: Map[String, Double],
      target: Double
  )
}
