package io.vaan.asare.modules

import org.http4s.HttpRoutes
import org.http4s.server.Router
import cats.effect._
import cats.syntax.all._
import org.http4s.HttpApp
import org.http4s.implicits._
import io.vaan.asare.modules.Algebras
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
  private val healthRoutes = new HealthRoutes[F](algrebras.healthCheck).routes

  private val rebalancerRoutesV1 = new RebalancerRoutes[F](algrebras.rebalancerV.v1).routes
  private val rebalancerRoutesV2 = new RebalancerRoutes[F](algrebras.rebalancerV.v2).routes

  private val openRoutersV1: HttpRoutes[F] =
    healthRoutes <+> rebalancerRoutesV1

  private val openRoutersV2: HttpRoutes[F] =
    healthRoutes <+> rebalancerRoutesV2

  private val routers: HttpRoutes[F] =
    Router(
      version.v1 -> openRoutersV1,
      version.v2 -> openRoutersV2
    )

  val httpApp: HttpApp[F] = routers.orNotFound
}
