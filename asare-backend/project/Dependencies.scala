import sbt._

object Dependencies {
  case object org {
    case object typelevel {
      val `cats-core` =
        "org.typelevel" %% "cats-core" % "2.4.2"

      val `cats-effect` =
        "org.typelevel" %% "cats-effect" % "2.3.3"

      val `kind-projector` =
        "org.typelevel" %% "kind-projector" % "0.11.0" cross CrossVersion.full
    }

    case object augustjune {
      val `context-applied` =
        "org.augustjune" %% "context-applied" % "0.1.4"
    }

    case object scalatest {
      val scalatest =
        "org.scalatest" %% "scalatest" % "3.2.5"
    }

    case object http4s {
      val http4sVersion = "0.21.18"

      val `http4s-dsl`          = "org.http4s" %% "http4s-dsl"          % http4sVersion
      val `http4s-blaze-server` = "org.http4s" %% "http4s-blaze-server" % http4sVersion
      val `http4s-blaze-client` = "org.http4s" %% "http4s-blaze-client" % http4sVersion
    }
  }
}
