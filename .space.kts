job("Docker: Build and Push") {
    startOn {
        gitPush {
            branchFilters {
                +"refs/head/latest"
            }
        }
    }

    docker {
        build {
            context: ".",
            file: "./Dockerfile",
            args["HTTP_PROXY"] = "http://0.0.0.0:9090",
            labels["vendor"] = "scattered-systems"
        }

        push("scattered-systems.registry.jetbrains.space/p/scattered-systems/containers/acme") {
            tags("0.0.\$JB_SPACE_EXECUTION_NUMBER", "latest")
        }
    }
    }
}