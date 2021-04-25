package io.vaan.asare.backend.http.routes

import cats._
import cats.syntax.all._
import org.http4s._
import org.http4s.dsl.Http4sDsl
import org.http4s.server.Router
import org.http4s.circe._
import io.circe.generic.auto._
import io.circe.syntax._
import org.http4s.circe.CirceEntityEncoder._
import io.vaan.asare.backend.programs.RebalancerP
import io.vaan.asare.backend.domain.rebalance._

final class RebalancerRoutesV3[F[_]: Monad: Defer: JsonDecoder](
    rebalancerP: RebalancerP[F]
) extends Http4sDsl[F] {
  private val httpRoutes: HttpRoutes[F] =
    HttpRoutes.of[F] {
      case request @ POST -> Root / "rebalance" =>
        request
          .asJsonDecode[RebalanceInput]
          .flatMap { input =>
            Ok(rebalancerP.rebalance(input))
          }
    }

  val routes: HttpRoutes[F] = Router(
    "rebel" -> httpRoutes
  )
}
