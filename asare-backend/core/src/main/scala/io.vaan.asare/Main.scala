package io.vaan.asare

import cats.effect.IOApp
import cats.effect.{ ExitCode, IO }

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    for {
      algrebras <- Algebras.make[IO]()
    } yield ExitCode.Success
}
