package io.vaan.asare.bot.http

import org.http4s.circe._
import io.circe.generic.auto._
import io.vaan.asare.bot.domain.rebalance.RebalanceOutput
import cats.effect.Sync
import org.http4s.EntityDecoder

package object clients {
  implicit def RebalanceOutputDecoder[F[_]: Sync]: EntityDecoder[F, RebalanceOutput] =
    jsonOf[F, RebalanceOutput]
}
