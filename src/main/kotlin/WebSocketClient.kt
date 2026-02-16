import com.fasterxml.jackson.module.kotlin.jacksonObjectMapper
import com.fasterxml.jackson.module.kotlin.readValue
import kotlinx.coroutines.channels.Channel
import kotlinx.coroutines.launch
import kotlinx.coroutines.runBlocking
import org.java_websocket.client.WebSocketClient
import org.java_websocket.handshake.ServerHandshake
import java.net.URI
import java.util.*

class RustWsClient(
    serverUri: URI,
    private val incoming: Channel<CommandResult>,
) : WebSocketClient(serverUri) {

    private val mapper = jacksonObjectMapper()

    override fun onOpen(handshakedata: ServerHandshake?) {
        println("Kotlin orchestrator connected to rust-core")
    }

    override fun onMessage(message: String?) {
        if (message == null) return
        runBlocking {
            try {
                val node: Map<String, Any?> = mapper.readValue(message)
                when (node["type"]) {
                    "result" -> {
                        val res: CommandResult = mapper.readValue(message)
                        incoming.send(res)
                    }
                    "error" -> println("rust-core error: $message")
                }
            } catch (e: Exception) {
                println("parse error: ${e.message}")
            }
        }
    }

    override fun onClose(code: Int, reason: String?, remote: Boolean) {
        println("ws closed: $code $reason")
    }

    override fun onError(ex: Exception?) {
        println("ws error: ${ex?.message}")
    }

    fun sendCommand(cmd: Command) {
        val payload = mapOf(
            "type" to "command",
            "kind" to when (cmd.kind) {
                is NeuralCachePurge -> "neural_cache_purge"
                is ContextExpand -> "context_expand"
                is SessionArchive -> "session_archive"
                is SessionSnapshot -> "session_snapshot"
                is SessionRecover -> "session_recover"
                is CodexValidate -> "codex_validate"
                is CodexCompress -> "codex_compress"
                is NodeRebalance -> "node_rebalance"
                is QueryTrace -> "query_trace"
                is QueryOptimize -> "query_optimize"
            },
            "role" to cmd.role.name,
            "scope" -> when (val s = cmd.scope) {
                is Scope.Node -> mapOf("kind" to "node", "name" to s.name)
                Scope.Cluster -> mapOf("kind" to "cluster")
                is Scope.Session -> mapOf("kind" to "session", "id" to s.id)
                Scope.Codex -> mapOf("kind" to "codex")
            },
            "id" to cmd.id
        )
        val json = mapper.writeValueAsString(payload)
        send(json)
    }
}
