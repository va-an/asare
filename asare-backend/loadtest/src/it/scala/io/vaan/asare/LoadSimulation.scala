package io.vaan.asare

import io.gatling.core.Predef._
import io.gatling.http.Predef._
import io.gatling.http.request._

import scala.concurrent.duration._
import scala.util.Random

class LoadSimulation extends Simulation {
  import LoadSimulation.genRebalanceV3Request

  val users   = System.getProperty("users").toInt
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
    def between: Double = Random.between(10_000, 100_000)

    s"""
{
  "currentPortfolio": {
    "A": ${Random.between(10_000, 100_000)},
    "B": ${Random.between(10_000, 100_000)},
    "C": ${Random.between(10_000, 100_000)}
  },
  "requiredAllocation": {
    "A": 33,
    "B": 33,
    "C": 34
  }
}
  """
  }
}
