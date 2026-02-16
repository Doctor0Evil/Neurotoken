import kotlinx.coroutines.*
import kotlinx.coroutines.channels.Channel
import java.net.URI
import java.util.*

class ClusterOrchestrator(
    private val client: RustWsClient,
    private val incoming: Channel<CommandResult>
) {
    fun start() = runBlocking {
        launch {
            for (res in incoming) {
                println("[cluster] result from rust-core: ${res.command_id} -> ${res.message}")
            }
        }

        // demo: send a few commands
        launch {
            val cmd1 = Command(
                id = UUID.randomUUID().toString(),
                kind = NeuralCachePurge,
                role = Role.admin,
                scope = Scope.Cluster
            )
            client.sendCommand(cmd1)

            val cmd2 = Command(
                id = UUID.randomUUID().toString(),
                kind = ContextExpand(amount = 512),
                role = Role.operator,
                scope = Scope.Cluster
            )
            client.sendCommand(cmd2)

            val cmd3 = Command(
                id = UUID.randomUUID().toString(),
                kind = SessionSnapshot,
                role = Role.admin,
                scope = Scope.Session(id = "session-1")
            )
            client.sendCommand(cmd3)
        }
    }
}
