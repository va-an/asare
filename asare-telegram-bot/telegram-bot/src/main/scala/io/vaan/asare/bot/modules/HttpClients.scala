package io.vaan.asare.bot.modules

import org.http4s._
import org.http4s.client.Client
import org.http4s.circe._
import org.http4s.circe.CirceEntityEncoder._
import cats.effect.Sync
import io.vaan.asare.bot.http.clients.RebalanceClient
import io.vaan.asare.bot.config.data._
import io.vaan.asare.bot.domain.rebalance._
import io.vaan.asare.bot.http.clients.LiveRebalanceClient

trait HttpClients[F[_]] {
  def rebalanceClient: RebalanceClient[F]
}

object HttpClients {
  def make[F[_]: Sync](
      config: Config,
      client: Client[F]
  ): F[HttpClients[F]] =
    Sync[F].delay(
      new HttpClients[F] {
        def rebalanceClient: RebalanceClient[F] =
          new LiveRebalanceClient[F](config, client)
      }
    )
}
