package io.vaan.asare.bot.scenarios

import canoe.api._
import canoe.syntax._
import cats.syntax.show._
import cats.effect.Sync
import io.vaan.asare.bot.http.clients.RebalanceClient
import io.vaan.asare.bot.domain.rebalance._
import io.vaan.asare.bot.algebras._

object RebalanceS {
  def apply[F[_]: TelegramClient: Sync](
      rebalanceClient: RebalanceClient[F],
      inputParser: InputParser[F]
  ): Scenario[F, Unit] =
    for {
      chat <- Scenario.expect(command("rebalance").chat)

      _ <- Scenario.eval(
        chat send "Enter your portfolio and desired allocation" // (show /example)")
      )

      userInput      <- Scenario.expect(text)
      rebalanceInput <- Scenario.eval(inputParser.parse(userInput)).attempt

      _ <- rebalanceInput.fold(
        e => Scenario.eval(chat send "Error with parse input, try again with /r command"),
        input =>
          Scenario
            .eval(rebalanceClient.rebalanceV3(input))
            .flatMap(output => Scenario.eval(chat send output.show))
      )
    } yield ()
}
