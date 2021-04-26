package io.vaan.asare.backend.config

import ciris._
import ciris.refined._
import eu.timepit.refined.api.Refined
import eu.timepit.refined.types.net.UserPortNumber
import eu.timepit.refined.types.string.NonEmptyString
import eu.timepit.refined.string._
import eu.timepit.refined.auto._
import cats.implicits._
import cats.effect.Async
import io.vaan.asare.backend.config.data._

object configuration {
  val config: ConfigValue[Effect, Config] =
    (
      (env("API_HOST") or prop("apiHost")).as[String Refined IPv4].option,
      (env("API_PORT") or prop("apiPort")).as[UserPortNumber].option
    ).parMapN { (apiHost, apiPort) =>
      Config(
        apiHost = apiHost getOrElse "0.0.0.0",
        apiPort = apiPort getOrElse 8090
      )
    }
}
