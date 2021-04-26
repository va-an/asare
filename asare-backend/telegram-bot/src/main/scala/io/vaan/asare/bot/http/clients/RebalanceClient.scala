package io.vaan.asare.bot.http.clients

import org.http4s._
import org.http4s.client.Client
import org.http4s.circe._
import io.circe.generic.auto._
import org.http4s.circe.CirceEntityEncoder._
import io.vaan.asare.bot.domain.rebalance._
import io.vaan.asare.bot.config.data
import cats.effect.Sync

trait RebalanceClient[F[_]] {
  def rebalanceV3(input: RebalanceInput): F[RebalanceOutput]
}

final class LiveRebalanceClient[F[_]: Sync](
    config: data.config.Config,
    client: Client[F]
) extends RebalanceClient[F] {
  override def rebalanceV3(input: RebalanceInput): F[RebalanceOutput] =
    client.expect[RebalanceOutput] {
      Request[F](Method.POST)
        .withUri(Uri.unsafeFromString(config.backendUrl.value + "v3/rebel/rebalance"))
        .withEntity(input)
        .withHeaders(Header("Content-Type", "application/json"))
    }
}
