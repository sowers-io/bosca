package io.bosca.workflow.ai

import dev.langchain4j.data.message.SystemMessage
import dev.langchain4j.data.message.UserMessage
import dev.langchain4j.model.chat.ChatLanguageModel
import dev.langchain4j.model.chat.request.ChatRequest
import dev.langchain4j.model.chat.request.ChatRequestParameters
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.decode
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import io.bosca.workflow.ai.schema.JsonSchema
import io.bosca.workflow.ext.toChatLanguageModel
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import dev.langchain4j.model.chat.request.json.JsonSchema as Schema

@Serializable
data class PromptContext(
    val tries: Int = 0,
)

class ExecutePrompt(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Execute Prompt",
            description = "Execute an AI Prompt",
            configuration = emptyMap<String, Any>().toOptional(),
            inputs = emptyList(),
            outputs = listOf(
                ActivityParameterInput(
                    OUTPUT_NAME,
                    ActivityParameterType.SUPPLEMENTARY
                )
            ),
        )
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        println("Executing prompt: ${job.prompts.firstOrNull()?.prompt?.prompt?.name}")
        val promptContext = getContext<PromptContext>(job).let {
            it.copy(tries = it.tries + 1)
        }
        if (promptContext.tries > 10) {
            error("too many tries")
        }

        setContext(job, promptContext)

        val modelId = job.models.firstOrNull()?.model?.model?.id ?: "default"
        val model = client.workflows.getModel(modelId) ?: error("missing model")

        val promptId = job.prompts.firstOrNull()?.prompt?.prompt?.id ?: error("missing prompt id")
        val prompt = client.workflows.getPrompt(promptId) ?: error("missing prompt")

        var userPrompt = prompt.userPrompt

        for (input in job.workflowActivity.workflowActivity.inputs) {
            val file = getInputSupplementaryFile(context, job, input.workflowActivityParameter)
            val text = withContext(Dispatchers.IO) { file?.readText() ?: "" }
            userPrompt = userPrompt.replace("{${input.workflowActivityParameter.name}}", text)
        }

        val userMessage: UserMessage = UserMessage.from(userPrompt)
        val systemMessage = SystemMessage.from(prompt.systemPrompt)

        var chatRequestBuilder: ChatRequest.Builder = ChatRequest.builder()

        if (prompt.schema != null && (prompt.schema as? Map<*, *>)?.isNotEmpty() == true) {
            prompt.schema.decode<JsonSchema>()?.let {
                chatRequestBuilder = chatRequestBuilder
                    .parameters(
                        ChatRequestParameters
                            .builder()
                            .responseFormat(
                                Schema.builder()
                                    .name(it.name.replace(" ", "_"))
                                    .rootElement(it.rootElement.toSchemaElement())
                                    .build()
                            )
                            .build()
                    )
            }
        }

        val chatRequest = chatRequestBuilder
            .messages(systemMessage, userMessage)
            .build()

        println("Sending Request for: ${job.prompts.firstOrNull()?.prompt?.prompt?.name}...")

        val chatModel: ChatLanguageModel = model.toChatLanguageModel(client)
        val chatResponse = chatModel.chat(chatRequest)
        val text = chatResponse.aiMessage().text().takeIf { it.isNotEmpty() && it != "null" } ?: ""

        println("Got response for: ${job.prompts.firstOrNull()?.prompt?.prompt?.name}...")

        setSupplementaryContents(
            job,
            OUTPUT_NAME,
            "Prompt Results",
            text,
            prompt.outputType,
        )
    }

    companion object {

        const val ID = "ai.prompt"
        const val OUTPUT_NAME = "supplementary"
    }
}