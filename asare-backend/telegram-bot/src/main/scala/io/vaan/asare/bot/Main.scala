package io.vaan.asare.bot

import ciris._
import canoe.api._
import canoe.syntax._
import cats.effect.{ ExitCode, IO, IOApp }
import cats.implicits._
import fs2.Stream
import io.chrisdavenport.log4cats.slf4j.Slf4jLogger
import io.vaan.asare.bot.config.Configuration._

object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    config.load[IO] flatMap { cfg =>
      for {
        logger <- Slf4jLogger.create[IO]
        _      <- logger.info(s"Loaded config: ${cfg.show}")
        _ <-
          Stream
            .resource(TelegramClient.global[IO](cfg.token.value))
            .flatMap(implicit client => Bot.polling[IO].follow(greetings))
            .compile
            .drain
            .as(ExitCode.Success)
      } yield ExitCode.Success
    }

  def greetings[F[_]: TelegramClient]: Scenario[F, Unit] =
    for {
      chat <- Scenario.expect(command("hi").chat)
      _    <- Scenario.eval(chat.send("Hello. What's your name?"))
      name <- Scenario.expect(text)
      _    <- Scenario.eval(chat.send(s"Nice to meet you, $name"))
    } yield ()
}
