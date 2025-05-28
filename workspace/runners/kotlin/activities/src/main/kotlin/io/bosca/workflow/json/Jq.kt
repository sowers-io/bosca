package io.bosca.workflow.json

import com.fasterxml.jackson.databind.JsonNode
import com.fasterxml.jackson.databind.ObjectMapper
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import net.thisptr.jackson.jq.JsonQuery
import net.thisptr.jackson.jq.Scope
import net.thisptr.jackson.jq.Version

@Serializable
data class JqConfiguration(
    val query: String
) {

    fun jq(json: String): List<JsonNode> {
        val query = JsonQuery.compile(query, Version.LATEST)
        val input = ObjectMapper().readTree(json)
        val scope = Scope.newEmptyScope()
        val nodes = mutableListOf<JsonNode>()
        query.apply(scope, input, nodes::add)
        return nodes
    }
}

class Jq(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "JQ",
            description = "Execute JQ Command",
            inputs = listOf(ActivityParameterInput(INPUT_NAME, ActivityParameterType.SUPPLEMENTARY)),
            outputs = listOf(ActivityParameterInput(OUTPUT_NAME, ActivityParameterType.SUPPLEMENTARY)),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = getConfiguration<JqConfiguration>(job)

        val file = getInputSupplementaryFile(context, job, INPUT_NAME)
        val json = withContext(Dispatchers.IO) { file?.readText() ?: "{}" }
        val nodes = configuration.jq(json)

        setSupplementaryContents(
            job,
            OUTPUT_NAME,
            "JQ Output",
            String.serializer(),
            nodes.joinToString("\r\n") {
                if (it.isArray) {
                    it.joinToString("\r\n") { it.asText() }
                } else {
                    it.asText()
                }
            },
        )
    }

    companion object {
        const val ID = "data.jq"
        const val INPUT_NAME = "supplementary"
        const val OUTPUT_NAME = "supplementary"
    }
}