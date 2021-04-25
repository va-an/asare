package io.vaan.asare.bot.config

import ciris._
import ciris.refined._
import eu.timepit.refined.types.net.UserPortNumber
import eu.timepit.refined.types.string.NonEmptyString
import eu.timepit.refined.auto._
import cats.implicits._
import cats.effect.Async
import cats.Show

object Configuration {
  final case class Config(
      token: NonEmptyString
  )

  implicit val showConfig: Show[Config] =
    (config: Config) => s"""
    Config(
      token = ${config.token}
    )
    """

  def config: ConfigValue[Config] =
    (
      (env("TOKEN") or prop("token")).as[NonEmptyString]
    ).map { token =>
      Config(
        token = token
      )
    }
}
