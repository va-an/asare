package io.vaan.asare.backend.modules

import cats.effect.Sync
import io.vaan.asare.backend.programs.RebalancerP
import io.vaan.asare.backend.algrebras

final class Programs[F[_]: Sync] private (
    algrebras: Algebras[F]
) {
  val rebalancerP: RebalancerP[F] =
    new RebalancerP[F](algrebras.rebalancerV.v2)
}

object Programs {
  def make[F[_]: Sync](algrebras: Algebras[F]): F[Programs[F]] =
    F.delay {
      new Programs[F](algrebras)
    }
}
