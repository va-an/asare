package io.vaan.asare.backend

import cats.effect.IOApp
import cats.effect.{ ExitCode, IO }
import org.http4s.server.blaze.BlazeServerBuilder
import org.typelevel.log4cats.Logger
import org.typelevel.log4cats.slf4j.Slf4jLogger
import scala.concurrent.ExecutionContext
import scala.concurrent.duration._
import io.vaan.asare.backend.config.Configuration._
import io.vaan.asare.backend.modules._

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    config.load[IO] flatMap { cfg =>
      for {
        logger    <- Slf4jLogger.create[IO]
        algrebras <- Algebras.make[IO]()
        programs  <- Programs.make[IO](algrebras)
        api       <- HttpApi.make[IO](algrebras, programs)
        _ <-
          BlazeServerBuilder[IO](ExecutionContext.global)
            .bindHttp(
              host = cfg.apiHost.value,
              port = cfg.apiPort.value
            )
            .withHttpApp(api.httpApp)
            .withMaxConnections(1024)
            .withResponseHeaderTimeout(60 seconds)
            .withIdleTimeout(120 seconds)
            .serve
            .compile
            .drain
      } yield ExitCode.Success
    }
}
