package io.vaan.asare.bot

import ciris._
import canoe.api._
import canoe.syntax._
import cats.effect.{ ExitCode, IO, IOApp }
import cats.implicits._
import fs2.Stream
import io.chrisdavenport.log4cats.slf4j.Slf4jLogger
import io.vaan.asare.bot.config.configuration._
import io.vaan.asare.bot.modules.HttpClients
import io.vaan.asare.bot.domain.rebalance._
import io.vaan.asare.bot.http.clients.RebalanceClient
import io.vaan.asare.bot.scenarios._
import io.vaan.asare.bot.algebras._

// TODO: show help for unknown command
// TODO: ru lang version and settings for this
// TODO: rebalance with target
// TODO: rebalance with price (not amount)
// TODO: parse amount with "_", example 123_456.78
// FIXME: blow up when input requared allocation is != 100
object Main extends IOApp {
  override def run(args: List[String]): IO[ExitCode] =
    config.load[IO] flatMap { cfg =>
      AppResources.make[IO]().use { res =>
        for {
          logger      <- Slf4jLogger.create[IO]
          _           <- logger.info(s"Loaded config: ${cfg.show}")
          clients     <- HttpClients.make[IO](cfg, res.client)
          inputParser <- LiveInputParser.make[IO] // TODO: to Algebras

          bot <-
            Stream
              .resource(TelegramClient.global[IO](cfg.token.value))
              .flatMap(implicit client =>
                Bot
                  .polling[IO]
                  .follow(
                    StartS[IO],
                    ExamplesS[IO],
                    RebalanceS[IO](clients.rebalanceClient, inputParser),
                    HelpS[IO],
                    AboutS[IO],
                    CommandsS[IO]
                  )
              )
              .compile
              .drain
              .as(ExitCode.Success)
        } yield bot
      }
    }
}
