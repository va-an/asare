package io.vaan.asare.bot

import org.http4s.client.Client
import cats.effect.Resource
import scala.concurrent.ExecutionContext
import org.http4s.client.blaze.BlazeClientBuilder
import scala.concurrent.duration._
import cats.effect.ConcurrentEffect

final case class AppResources[F[_]](
    client: Client[F]
)

object AppResources {
  def make[F[_]: ConcurrentEffect](
  ): Resource[F, AppResources[F]] = {
    def mkHttpClient(): Resource[F, Client[F]] =
      BlazeClientBuilder[F](ExecutionContext.global)
        .withConnectTimeout(60 seconds)
        .withRequestTimeout(60 seconds)
        .withIdleTimeout(120 seconds)
        .resource

    mkHttpClient().map(client => AppResources(client))
  }
}
