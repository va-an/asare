package io.vaan.asare.http.routes

import io.vaan.asare.algrebras.HealthCheck
import org.http4s.dsl.Http4sDsl
import org.http4s.server.Router
import org.http4s._
import cats._

final class HealthRoutes[F[_]: Defer: Monad](
    healthCheck: HealthCheck[F]
) extends Http4sDsl[F] {
  private val httpRoutes: HttpRoutes[F] =
    HttpRoutes.of[F] {
      case GET -> Root => Ok(healthCheck.status)
    }

  val routes: HttpRoutes[F] = Router(
    "health" -> httpRoutes
  )
}
