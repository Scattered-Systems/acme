job("Docker: Build and Push") {
    startOn {
        gitPush {
            branchFilters {
                +"refs/head/master"
            }
        }
    }

    docker {
        build {
            context: "."
            file: "./Dockerfile"
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scsys/containers/acme") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
}