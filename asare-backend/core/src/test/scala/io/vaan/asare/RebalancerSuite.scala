package io.vaan.asare

import org.scalatest.flatspec.AnyFlatSpec
import org.scalatest.matchers.should.Matchers._
import cats._
import cats.effect.IO
import io.vaan.asare.domain.rebalance._
import io.vaan.asare.algrebras.rebalancer._

class RebalancerSuite extends AnyFlatSpec {
  val currentPortfolio: Map[String, Double] = Map(
    "A" -> 75_000,
    "B" -> 100_000,
    "C" -> 125_000
  )

  val expectedAllocation: Map[String, Double] = Map(
    "A" -> 33,
    "B" -> 33,
    "C" -> 34
  )

  val rebalanceInput = RebalanceInput(
    actualPortfolio = currentPortfolio,
    expectedAllocation = expectedAllocation,
    target = 300_000
  )

  val rebalancer = RebalancerA
    .makeV[IO]()
    .unsafeRunSync()
    .v1

  it should "calculate current allocation" in {
    val result = rebalancer
      .calcCurrentAllocation(currentPortfolio)
      .unsafeRunSync()

    result.values.sum should equal(100)

    result("A") should equal(25.00 +- 0.01)
    result("B") should equal(33.33 +- 0.01)
    result("C") should equal(41.66 +- 0.01)
  }

  it should "calculate expected allocation" in {
    val result = rebalancer
      .calcExpectedPortfolio(rebalanceInput)
      .unsafeRunSync()

    result.values.sum should equal(300_000)

    result("A") should equal(99_000.0 +- 0.01)
    result("B") should equal(99_000.0 +- 0.01)
    result("C") should equal(102_000.0 +- 0.01)
  }

  it should "calculate purchase" in {
    val result = rebalancer.calcPurchase(rebalanceInput)

    // TODO: write test
    assert(true == true)
  }
}
