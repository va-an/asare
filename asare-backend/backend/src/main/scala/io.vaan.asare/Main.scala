package io.vaan.asare

import cats.effect.IOApp
import cats.effect.{ ExitCode, IO }
import org.http4s.server.blaze.BlazeServerBuilder
import org.typelevel.log4cats.Logger
import org.typelevel.log4cats.slf4j.Slf4jLogger
import io.vaan.asare.config.Config
import io.vaan.asare.modules._
import scala.concurrent.ExecutionContext
import scala.concurrent.duration._

// TODO: generate API docs (tapir, rho)
// TODO: get prices from some public API and make version of API for this
object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    for {
      logger    <- Slf4jLogger.create[IO]
      algrebras <- Algebras.make[IO]()
      programs  <- Programs.make[IO](algrebras)
      api       <- HttpApi.make[IO](algrebras, programs)
      _ <-
        BlazeServerBuilder[IO](ExecutionContext.global)
          .bindHttp(
            host = Config.httpHost,
            port = Config.httpPort
          )
          .withHttpApp(api.httpApp)
          .withMaxConnections(1024)
          .withResponseHeaderTimeout(60 seconds)
          .serve
          .compile
          .drain
    } yield ExitCode.Success
}
