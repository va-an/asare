package io.vaan.asare.modules

import io.vaan.asare.programs.RebalancerP
import cats.effect.Sync

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
