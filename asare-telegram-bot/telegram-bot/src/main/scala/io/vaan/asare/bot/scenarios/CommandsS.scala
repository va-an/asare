package io.vaan.asare.bot.scenarios

import canoe.api._
import canoe.syntax._
import canoe.methods.commands.SetMyCommands
import canoe.models.BotCommand

object CommandsS {
  def apply[F[_]: TelegramClient]: Scenario[F, Unit] =
    for {
      commands <- Scenario.pure(
        List(
          BotCommand("rebalance", "calculate rebalance"),
          BotCommand("example", "show input example"),
          BotCommand("help", "show help page"),
          BotCommand("about", "github page")
        )
      )
      _ <- Scenario.eval(SetMyCommands(commands).call)
    } yield ()
}
