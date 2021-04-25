package io.vaan.asare.backend.domain

object rebalance {
  type Portfolio = Map[String, Double]

  final case class RebalanceInput(
      currentPortfolio: Portfolio,
      requiredAllocation: Portfolio,
      target: Option[Double]
  )

  final case class RebalanceOutput(
      currentAllocation: Portfolio,
      requiredOperations: Portfolio
  )
}
