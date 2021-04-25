package io.vaan.asare.backend.modules

import io.vaan.asare.backend.algrebras.rebalancer._
import io.vaan.asare.backend.algrebras._
import cats._
import cats.implicits._
import cats.effect.Sync

final class Algebras[F[_]] private (
    val rebalancerV: RebalancerV[F],
    val healthCheck: HealthCheck[F]
)

object Algebras {
  def make[F[_]: Sync](): F[Algebras[F]] =
    for {
      rebalancerV <- RebalancerA.makeV[F]()
      healthCheck <- HealthCheck.make[F]()
    } yield new Algebras[F](
      rebalancerV,
      healthCheck
    )
}
