package io.vaan.asare.modules

import org.http4s.HttpRoutes
import org.http4s.server.Router
import cats.effect._
import cats.syntax.all._
import org.http4s.HttpApp
import org.http4s.server.middleware._
import org.http4s.implicits._
import io.vaan.asare.modules.Algebras
import io.vaan.asare.algrebras.HealthCheck
import io.vaan.asare.http.routes._

object HttpApi {
  def make[F[_]: Concurrent: Timer](
      algrebras: Algebras[F],
      programs: Programs[F]
  ): F[HttpApi[F]] =
    Sync[F].delay(
      new HttpApi[F](algrebras, programs)
    )
}

final class HttpApi[F[_]: Concurrent] private (
    algrebras: Algebras[F],
    programs: Programs[F]
) {
  private val healthRoutes = new HealthRoutes[F](algrebras.healthCheck).routes

  private val rebalancerRoutesV1 = new RebalancerRoutes[F](algrebras.rebalancerV.v1).routes
  private val rebalancerRoutesV2 = new RebalancerRoutes[F](algrebras.rebalancerV.v2).routes
  private val rebalancerRoutesV3 = new RebalancerRoutesV3[F](programs.rebalancerP).routes

  private val openRoutersV1: HttpRoutes[F] =
    healthRoutes <+> rebalancerRoutesV1

  private val openRoutersV2: HttpRoutes[F] =
    healthRoutes <+> rebalancerRoutesV2

  private val openRoutersV3: HttpRoutes[F] =
    healthRoutes <+> rebalancerRoutesV3

  private val routers: HttpRoutes[F] =
    Router(
      version.v1 -> openRoutersV1,
      version.v2 -> openRoutersV2,
      version.v3 -> openRoutersV3
    )

  // format: off
  private val middleware: HttpRoutes[F] => HttpRoutes[F] = { 
    http: HttpRoutes[F] =>
      AutoSlash(http)
  }

  private val loggers: HttpApp[F] => HttpApp[F] = { 
    http: HttpApp[F] =>
      RequestLogger.httpApp(false, false)(http)
  }

  // format: on
  val httpApp: HttpApp[F] = loggers(middleware(routers).orNotFound)
}
