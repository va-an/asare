package io.vaan.asare.bot.domain

import cats.Show
import cats.implicits._

object rebalance {
  type Portfolio = Map[String, Double]

  val formatter = java.text.NumberFormat.getInstance()

  final case class RebalanceInput(
      current_portfolio: Portfolio,
      required_allocation: Portfolio,
      target: Option[Double]
  )

  final case class RebalanceOutput(
      current_allocation: Portfolio,
      required_operations: Portfolio
  )

  implicit val rebalanceOutputBotShow: Show[RebalanceOutput] = {
    def mapToString(m: Map[String, Double]) =
      m.map { case (ticker: String, value: Double) => ticker -> (formatter format value) }
        .map(_.productIterator.mkString("\t: "))
        .mkString("\n")

    (output: RebalanceOutput) =>
      s"""
      |current allocation:
      |${mapToString(output.current_allocation)}

      |required operations:
      |${mapToString(output.required_operations)}
      """.stripMargin
  }
}
