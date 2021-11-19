lazy val root = (project in file(".")).settings(
  inThisBuild(
    List(
      organization := "csixteen.leetcode",
      scalaVersion := "2.13.6"
    )
  ),
  name := "scala"
)

libraryDependencies += "org.scalatest" %% "scalatest" % "3.1.0" % Test
