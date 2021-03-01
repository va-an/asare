package `io.vaan.asare`

import io.vaan.asare.algrebras.Rebalancer
import cats._
import cats.implicits._

final class Algebras[F[_]] private (
    val rebalancer: Rebalancer[F]
)

object Algebras {
  def make[F[_]: Applicative](): F[Algebras[F]] =
    for {
      rebalancer <- Rebalancer.make[F]()
    } yield new Algebras[F](rebalancer)
}
