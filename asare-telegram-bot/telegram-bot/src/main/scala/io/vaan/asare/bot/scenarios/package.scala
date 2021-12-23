package io.vaan.asare.bot

package object scanarios {
  val help = """
    |Bot for calculation asset allocation rebalance.
    |
    |Commands:
    |/rebalance - calculate rebalance
    |/example - example input message for /rebalance
    |/help - help
    |/about - for feature request and bug reports
    """.stripMargin.strip

  val exampleMessage = """
    |Input format:
    |<ticker> <current amount> <requered allocation>
    """.stripMargin.strip

  val example = """
    |A 75000 33
    |B 100000 33
    |C 125000 34
    """.stripMargin.strip

  val about = """
  |This bot created with love and open source.
  |Code here - https://github.com/va-anufriev/asare
  |Feel free create a issues with feature or bugfix requests!
  """.stripMargin.strip
}
