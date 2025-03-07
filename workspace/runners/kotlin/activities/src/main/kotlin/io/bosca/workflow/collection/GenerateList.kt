package io.bosca.workflow.collection

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.graphql.type.FindAttributeInput
import io.bosca.util.toJsonElement
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable

@Serializable
data class GenerateListConfiguration(
    val filter: List<GenerateFindAttribute> = emptyList(),
)

@Serializable
data class GenerateFindAttribute(
    val key: String,
    val `value`: String,
)

class GenerateList(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Generate Collection List",
            description = "Generate List of Collection Names and IDs",
            inputs = emptyList(),
            outputs = listOf(
                ActivityParameterInput(OUTPUT_NAME, ActivityParameterType.SUPPLEMENTARY)
            ),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val find = getConfiguration<GenerateListConfiguration>(job)
        val results = mutableListOf<Map<String, String>>()
        var offset = 0
        while (true) {
            val result = client.collections.findCollections(
                find.filter.map { FindAttributeInput(it.key, it.`value`) },
                offset,
                100
            )
            for (collection in result) {
                results.add(mapOf("name" to collection.name, "id" to collection.id))
            }
            if (result.isEmpty()) break
            offset += 100
        }
        setSupplementaryContents(
            job,
            OUTPUT_NAME,
            "Collection List",
            results.toJsonElement().toString(),
            "application/json"
        )
    }

    companion object {
        const val ID = "collection.generate.list"
        const val OUTPUT_NAME = "collections"
    }
}