package io.vaan.asare.config

import ciris._
import ciris.refined._
import eu.timepit.refined.types.net.UserPortNumber
import eu.timepit.refined.types.string.NonEmptyString
import eu.timepit.refined.auto._
import cats.implicits._
import cats.effect.Async
import eu.timepit.refined.string

case class Config(
    apiHost: NonEmptyString,
    apiPort: UserPortNumber
)

object load {
  def apply[F[_]: Async]: F[Config] =
    (
      (env("API_HOST") or prop("apiHost")).as[NonEmptyString].option,
      (env("API_PORT") or prop("apiPort")).as[UserPortNumber].option
    ).parMapN { (apiHost, apiPort) =>
        Config(
          apiHost = apiHost getOrElse "0.0.0.0",
          apiPort = apiPort getOrElse 8090
        )
      }
      .load[F]
}
