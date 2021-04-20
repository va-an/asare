package io.vaan.asare.algrebras

import cats._
import cats.syntax.all._

trait HealthCheck[F[_]] {
  def status: F[String]
}

object HealthCheck {
  def make[F[_]: Applicative](): F[HealthCheck[F]] =
    F.pure {
      new HealthCheck[F] {
        override def status: F[String] =
          F.pure("Ok")
      }
    }
}
