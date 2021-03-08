package io.vaan.asare

import cats.effect.IOApp
import cats.effect.{ ExitCode, IO }
import org.http4s.server.blaze.BlazeServerBuilder
import scala.concurrent.ExecutionContext

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    for {
      algrebras <- Algebras.make[IO]()
      api       <- HttpApi.make[IO](algrebras)
      _ <-
        BlazeServerBuilder[IO](ExecutionContext.global)
          .bindHttp(
            port = 8080, // TODO: get from config
            host = "0.0.0.0"
          )
          .withHttpApp(api.httpApp)
          .serve
          .compile
          .drain
    } yield ExitCode.Success
}
