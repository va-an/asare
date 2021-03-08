package io.vaan.asare

import org.http4s.HttpRoutes
import org.http4s.server.Router
import cats.effect._
import cats.syntax.all._
import org.http4s.HttpApp
import org.http4s.implicits._
import io.vaan.asare.Algebras
import io.vaan.asare.algrebras.HealthCheck
import io.vaan.asare.http.routes._

object HttpApi {
  def make[F[_]: Concurrent: Timer](
      algrebras: Algebras[F]
  ): F[HttpApi[F]] =
    Sync[F].delay(
      new HttpApi[F](algrebras)
    )
}

final class HttpApi[F[_]: Concurrent] private (
    algrebras: Algebras[F]
) {
  private val healthRoutes: HttpRoutes[F] =
    new HealthRoutes[F](algrebras.healthCheck).routes

  private val openRouters: HttpRoutes[F] =
    healthRoutes

  private val routers: HttpRoutes[F] =
    Router(
      version.v1 -> openRouters
    )

  val httpApp: HttpApp[F] = routers.orNotFound
}
