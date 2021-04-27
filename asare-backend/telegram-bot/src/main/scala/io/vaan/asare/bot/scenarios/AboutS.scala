package io.vaan.asare.bot.scenarios

import canoe.api._
import canoe.syntax._
import io.vaan.asare.bot.scanarios._

object AboutS {
  def apply[F[_]: TelegramClient]: Scenario[F, Unit] =
    for {
      chat <- Scenario.expect((command("about")).chat)
      _    <- Scenario.eval(chat send about)
    } yield ()
}
