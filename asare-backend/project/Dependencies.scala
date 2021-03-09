import sbt._

object Dependencies {
  object Versions {
    val cats           = "2.4.2"
    val catsEffect     = "2.3.3"
    val http4s         = "0.21.18"
    val scalaTest      = "3.2.5"
    val contextApplied = "0.1.4"
    val circe          = "0.13.0"
  }

  object Libraries {
    def http4s(artifact: String): ModuleID = "org.http4s" %% artifact % Versions.http4s
    def circe(artifact: String): ModuleID  = "io.circe"   %% artifact % Versions.circe

    val cats       = "org.typelevel" %% "cats-core"   % Versions.cats
    val catsEffect = "org.typelevel" %% "cats-effect" % Versions.catsEffect

    val http4sDsl    = http4s("http4s-dsl")
    val http4sServer = http4s("http4s-blaze-server")
    val http4sClient = http4s("http4s-blaze-client")
    val http4sCirce  = http4s("http4s-circe")

    val circeGeneric = circe("circe-generic")

    val scalaTest = "org.scalatest" %% "scalatest" % Versions.scalaTest

    val contextApplied = "org.augustjune" %% "context-applied" % Versions.contextApplied
  }
}
