package io.vaan.asare.http.routes

import cats._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.Http4sDsl
import org.http4s.server.Router
import org.http4s.circe._
import io.circe.generic.auto._
import io.circe.syntax._
import org.http4s.circe.CirceEntityEncoder._
import io.vaan.asare.algrebras.rebalancer.RebalancerA
import io.vaan.asare.domain.rebalance.Portfolio

final class RebalancerRoutes[F[_]: Monad: Defer: JsonDecoder](
    rebalancer: RebalancerA[F]
) extends Http4sDsl[F] {
  private val httpRoutes: HttpRoutes[F] =
    HttpRoutes.of[F] {
      case request @ POST -> Root / "allocation" =>
        request
          .asJsonDecode[Portfolio]
          .flatMap { current =>
            Ok(rebalancer.calcCurrentAllocation(current))
          }
    }

  val routes: HttpRoutes[F] = Router(
    "rebalance" -> httpRoutes
  )
}
