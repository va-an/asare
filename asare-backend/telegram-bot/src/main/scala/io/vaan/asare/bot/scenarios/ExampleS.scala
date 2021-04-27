package io.vaan.asare.bot.scenarios

import canoe.api._
import canoe.syntax._
import cats.syntax.show._
import io.vaan.asare.bot.scanarios._

object ExampleS {
  def apply[F[_]: TelegramClient]: Scenario[F, Unit] =
    for {
      chat <- Scenario.expect((command("example") orElse command("e")).chat)

      _ <- Scenario.eval(chat send exampleMessage)
      _ <- Scenario.eval(chat send example)
    } yield ()
}
