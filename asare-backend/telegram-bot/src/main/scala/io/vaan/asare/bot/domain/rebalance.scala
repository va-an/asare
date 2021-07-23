package io.vaan.asare.bot.domain

import cats.Show
import cats.implicits._

object rebalance {
  type Portfolio = Map[String, Double]

  val formatter = java.text.NumberFormat.getInstance()

  final case class RebalanceInput(
      currentPortfolio: Portfolio,
      requiredAllocation: Portfolio,
      target: Option[Double]
  )

  final case class RebalanceOutput(
      currentAllocation: Portfolio,
      requiredOperations: Portfolio
  )

  implicit val rebalanceOutputBotShow: Show[RebalanceOutput] = {
    def mapToString(m: Map[String, Double]) =
      m.map { case (ticker: String, value: Double) => ticker -> (formatter format value) }
        .map(_.productIterator.mkString("\t: "))
        .mkString("\n")

    (output: RebalanceOutput) => s"""
      |current allocation:
      |${mapToString(output.currentAllocation)}

      |required operations:
      |${mapToString(output.requiredOperations)}
      """.stripMargin
  }
}
