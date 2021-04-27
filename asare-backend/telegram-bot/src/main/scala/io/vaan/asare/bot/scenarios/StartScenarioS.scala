package io.vaan.asare.bot.scenarios

import canoe.api._
import canoe.syntax._
import io.vaan.asare.bot.scanarios._

object StartS {
  def apply[F[_]: TelegramClient]: Scenario[F, Unit] =
    for {
      chat <- Scenario.expect((command("start")).chat)
      _    <- Scenario.eval(chat send "Hello!")
      _    <- Scenario.eval(chat send help)
    } yield ()
}
