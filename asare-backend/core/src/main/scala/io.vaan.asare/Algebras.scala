package io.vaan.asare

import io.vaan.asare.algrebras._
import cats._
import cats.implicits._

final class Algebras[F[_]] private (
    val rebalancer: Rebalancer[F],
    val healthCheck: HealthCheck[F]
)

object Algebras {
  def make[F[_]: Monad](): F[Algebras[F]] =
    for {
      rebalancer  <- Rebalancer.make[F]()
      healthCheck <- HealthCheck.make[F]()
    } yield new Algebras[F](rebalancer, healthCheck)
}
