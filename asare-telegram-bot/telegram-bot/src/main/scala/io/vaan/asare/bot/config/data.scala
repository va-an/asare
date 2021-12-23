package io.vaan.asare.bot.config

import eu.timepit.refined.types.net.UserPortNumber
import eu.timepit.refined.types.string.NonEmptyString
import eu.timepit.refined.api.Refined
import eu.timepit.refined.string.Url
import cats.Show

object data {
  final case class Config(
      token: NonEmptyString,
      backendUrl: String Refined Url
  )

  implicit val showConfig: Show[Config] =
    (config: Config) => s"""
      |Config(
      |  token       = ${config.token},
      |  backendUrl  = ${config.backendUrl}
      |)
    """.stripMargin
}
