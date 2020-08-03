val dottyVersion = "0.25.0"
scalaSource in Compile := baseDirectory.value / "src"

enablePlugins(GraalVMNativeImagePlugin)
graalVMNativeImageOptions ++= Seq(
        "--initialize-at-build-time",
        "-Ycheck-init"
    )

lazy val root = project
    .in(file("."))
    .settings(
        name := "NE",
        version := "1.0.0",
        scalaVersion := dottyVersion,
        scalacOptions += "-Yexplicit-nulls"
    )
addCommandAlias("native-image", "graalvm-native-image:packageBin")
javaHome := Some(file("~/graal"))
