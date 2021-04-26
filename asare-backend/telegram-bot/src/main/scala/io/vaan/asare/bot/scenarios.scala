package io.vaan.asare.bot

import canoe.api._
import canoe.syntax._
import cats.implicits._
import canoe.models.messages.TextMessage
import cats.effect.Sync
import io.vaan.asare.bot.http.clients.RebalanceClient
import io.vaan.asare.bot.domain.rebalance._
import io.vaan.asare.bot.algebras._

object scenarios {
  def rebalanceScenario[F[_]: TelegramClient: Sync](
      rebalanceClient: RebalanceClient[F],
      inputParser: InputParser[F]
  ): Scenario[F, Unit] =
    for {
      chat <- Scenario.expect((command("rebalance") orElse command("r")).chat)

      _ <- Scenario.eval(
        chat send "Enter your portfolio and desired allocation" // (show /example)")
      )

      userInput      <- Scenario.expect(text)
      rebalanceInput <- Scenario.eval(inputParser.parse(userInput)).attempt

      // TODO: show "parse error message" to user
      _ <- rebalanceInput.fold(
        e => Scenario.eval(processError(e)).flatMap(m => Scenario.eval(chat send m)),
        input =>
          Scenario
            .eval(rebalanceClient.rebalanceV3(input))
            .flatMap(output => Scenario.eval(chat send output.show))
      )
    } yield ()

  def exampleScenario[F[_]: TelegramClient]: Scenario[F, Unit] = {
    val example = """A 75000 33
                    |B 100000 33
                    |C 125000 34""".stripMargin

    for {
      chat <- Scenario.expect((command("example") orElse command("e")).chat)
      _    <- Scenario.eval(chat send example)
    } yield ()
  }

  def processError[F[_]: Sync](e: Throwable): F[String] =
    Sync[F].pure("Something went wrong while making your order. Please try again.")
}
