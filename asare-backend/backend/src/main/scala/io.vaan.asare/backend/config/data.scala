package io.vaan.asare.backend.config

import eu.timepit.refined.api.Refined
import eu.timepit.refined.types.net.UserPortNumber
import eu.timepit.refined.types.string.NonEmptyString
import eu.timepit.refined.string._
import cats.Show

object data {
  final case class Config(
      apiHost: String Refined IPv4,
      apiPort: UserPortNumber
  )

  implicit val showConfig: Show[Config] =
    (config: Config) => s"""
    Config(
      apiHost = ${config.apiHost},
      apiPort = ${config.apiPort}
    )
    """
}
