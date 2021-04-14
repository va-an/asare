package io.vaan.asare

import io.gatling.core.Predef._
import io.gatling.http.Predef._
import io.gatling.http.request._

import scala.concurrent.duration._
import scala.util.Random

class LoadSimulation extends Simulation {
  import LoadSimulation.genRebalanceV3Request

  val users   = System.getProperty("users", "1").toInt
  val baseUrl = "http://localhost:8090"

  val scn = scenario("rebalance v3")
    .exec(
      repeat(10) {
        exec(
          http("rebalance v3 request")
            .post(baseUrl + "/v3/rebel/rebalance")
            .body(StringBody(_ => genRebalanceV3Request))
            .header("Content-Type", "application/json")
        ).pause(1 second, 2 seconds)
      }
    )

  setUp(
    scn.inject(rampUsers(users) during (10 seconds))
  )
}

object LoadSimulation {
  def genRebalanceV3Request: String = {
    def genBetween: Double = Random.between(10_000, 100_000)

    val allocation: (Int, Int, Int) = {
      val a1: Int = Random.between(25, 40)
      val a2: Int = Random.between(25, 40)
      val a3: Int = 100 - (a1 + a2)

      (a1, a2, a3)
    }

    s"""
{
  "currentPortfolio": {
    "A": $genBetween,
    "B": $genBetween,
    "C": $genBetween
  },
  "requiredAllocation": {
    "A": ${allocation._1},
    "B": ${allocation._2},
    "C": ${allocation._3}
  }
}
  """
  }
}
