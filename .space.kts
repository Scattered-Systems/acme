job("Docker: Build and Push") {
    startOn {
        gitPush {
            branchFilter {
                +"refs/heads/master"
            }
        }
    }

    docker {
        build {
            context: "."
            customPlatform = "linux/arm"
            file: "./Dockerfile"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/acme") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}