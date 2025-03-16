package io.bosca.workflow.general

import com.fasterxml.jackson.databind.ObjectMapper
import com.fasterxml.jackson.datatype.jsr310.JavaTimeModule
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable
import org.graalvm.polyglot.*;

@Serializable
data class IfConfiguration(
    val expression: String,
    val workflows: List<String> = emptyList(),
)

@Serializable
data class IfContext(val executed: Boolean = false)

class If(client: Client) : Activity(client) {
    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Run a child workflow if a condition is met",
            description = "Run a child workflow if a condition is met",
            inputs = emptyList(),
            outputs = emptyList(),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val ctx = getContext<IfContext>(job)
        if (ctx.executed) return
        val cfg = getConfiguration<IfConfiguration>(job)
        val jsContext = Context.newBuilder("js")
            .allowHostAccess(HostAccess.ALL) //allows access to all Java classes
            .allowHostClassLookup { className -> true }
            .out(System.out)
            .err(System.err)
            .build()
        val ow = ObjectMapper().registerModules(JavaTimeModule()).writer().withDefaultPrettyPrinter()
        val json = ow.writeValueAsString(job)
        jsContext.getBindings("js").putMember("job", json)
        val response = jsContext.eval("js", cfg.expression)
        if (response.asBoolean()) {
            client.workflows.enqueueChildWorkflows(
                cfg.workflows,
                job.id
            )
            setContext(job, IfContext(executed = true))
        }
    }

    companion object {

        const val ID = "workflow.general.if"
        const val INPUT_NAME = "supplementary"
        const val OUTPUT_NAME = "supplementary"
    }
}