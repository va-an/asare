package io.vaan.asare

import cats.effect.IOApp
import cats.effect.{ ExitCode, IO }
import org.http4s.server.blaze.BlazeServerBuilder
import scala.concurrent.ExecutionContext
import io.vaan.asare.config.Config
import io.vaan.asare.modules._

// TODO: add logging
object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    for {
      algrebras <- Algebras.make[IO]()
      api       <- HttpApi.make[IO](algrebras)
      _ <-
        BlazeServerBuilder[IO](ExecutionContext.global)
          .bindHttp(
            port = Config.httpPort,
            host = Config.httpHost
          )
          .withHttpApp(api.httpApp)
          .serve
          .compile
          .drain
    } yield ExitCode.Success
}
