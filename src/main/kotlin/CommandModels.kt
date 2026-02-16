import com.fasterxml.jackson.annotation.JsonSubTypes
import com.fasterxml.jackson.annotation.JsonTypeInfo

enum class Role { admin, operator, observer }

sealed class Scope {
    data class Node(val name: String): Scope()
    object Cluster: Scope()
    data class Session(val id: String): Scope()
    object Codex: Scope()
}

@JsonTypeInfo(use = JsonTypeInfo.Id.NAME, property = "kind")
@JsonSubTypes(
    JsonSubTypes.Type(value = NeuralCachePurge::class, name = "neural_cache_purge"),
    JsonSubTypes.Type(value = ContextExpand::class, name = "context_expand"),
    JsonSubTypes.Type(value = SessionArchive::class, name = "session_archive"),
    JsonSubTypes.Type(value = SessionSnapshot::class, name = "session_snapshot"),
    JsonSubTypes.Type(value = SessionRecover::class, name = "session_recover"),
    JsonSubTypes.Type(value = CodexValidate::class, name = "codex_validate"),
    JsonSubTypes.Type(value = CodexCompress::class, name = "codex_compress"),
    JsonSubTypes.Type(value = NodeRebalance::class, name = "node_rebalance"),
    JsonSubTypes.Type(value = QueryTrace::class, name = "query_trace"),
    JsonSubTypes.Type(value = QueryOptimize::class, name = "query_optimize")
)
sealed class CommandKind

object NeuralCachePurge: CommandKind()
data class ContextExpand(val amount: Int): CommandKind()
object SessionArchive: CommandKind()
object SessionSnapshot: CommandKind()
object SessionRecover: CommandKind()
object CodexValidate: CommandKind()
object CodexCompress: CommandKind()
object NodeRebalance: CommandKind()
object QueryTrace: CommandKind()
object QueryOptimize: CommandKind()

data class Command(
    val id: String,
    val kind: CommandKind,
    val role: Role,
    val scope: Scope
)

data class CommandResult(
    val command_id: String,
    val ok: Boolean,
    val message: String
)

sealed class WsMessage {
    data class CommandMsg(val type: String = "command", val command: Command): WsMessage()
    data class ResultMsg(val type: String = "result", val command_id: String, val ok: Boolean, val message: String): WsMessage()
    data class ErrorMsg(val type: String = "error", val message: String): WsMessage()
}
