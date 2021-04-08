package io.vaan.asare

import io.gatling.core.Predef._
import io.gatling.http.Predef._

import scala.concurrent.duration._

class LoadSimulation extends Simulation {
  val httpProtocol = http
    .baseUrl("http://localhost:8090")
    .acceptHeader("text/html,application/xhtml+xml,application/xml;q=0.9,*/*;q=0.8")
    .acceptEncodingHeader("gzip, deflate")
    .acceptLanguageHeader("en-US,en;q=0.5")
    .userAgentHeader(
      "Mozilla/5.0 (Macintosh; Intel Mac OS X 10.8; rv:16.0) Gecko/20100101 Firefox/16.0"
    )

  val scn = scenario("Yeah Boi")
    .exec(
      http("request_1")
        .get("/v1/health")
    )
    .pause(7)

  setUp(scn.inject(atOnceUsers(10)).protocols(httpProtocol))
}
