package io.vaan.asare.config

import eu.timepit.refined.types.net.UserPortNumber
import eu.timepit.refined.types.string.NonEmptyString

final case class Config(
    apiHost: NonEmptyString,
    apiPort: UserPortNumber
)
