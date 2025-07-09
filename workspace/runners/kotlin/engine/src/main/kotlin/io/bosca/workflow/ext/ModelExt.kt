package io.bosca.workflow.ext

import dev.langchain4j.model.chat.ChatLanguageModel
import dev.langchain4j.model.embedding.EmbeddingModel
import dev.langchain4j.model.embedding.onnx.allminilml6v2.AllMiniLmL6V2EmbeddingModel
import dev.langchain4j.model.googleai.GoogleAiGeminiChatModel
import dev.langchain4j.model.openai.OpenAiChatModel
import io.bosca.api.Client
import io.bosca.graphql.fragment.Model
import io.bosca.models.GoogleConfiguration
import io.bosca.models.OllamaConfiguration
import io.bosca.models.OpenAIConfiguration
import io.bosca.util.ModelTypes
import io.bosca.util.decode
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.modules.SerializersModule
import kotlinx.serialization.modules.polymorphic
import java.time.Duration

fun Model.toEmbeddingModel(): EmbeddingModel {
    when (type) {
        "embedding" -> {
            when (name) {
                "all-mini-lm-l6-v2" -> return AllMiniLmL6V2EmbeddingModel()
                else -> throw UnsupportedOperationException("Unsupported embedding name: $name")
            }
        }

        else -> throw UnsupportedOperationException("Unsupported model type: $type")
    }
}

suspend fun Model.toChatLanguageModel(client: Client): ChatLanguageModel {
    val configuration = configuration.decode<LanguageModelConfiguration>()
    
    return when (type) {
        ModelTypes.OpenAI -> {
            val key = configuration?.apiKey
                ?: client.configurations.get<OpenAIConfiguration>(OpenAIConfiguration.KEY)?.key
                ?: error("apiKey missing")
            OpenAiChatModel.builder()
                .apiKey(key)
                .modelName(configuration?.modelName ?: name)
                .timeout(Duration.ofMinutes(30))
                .logRequests(true)
                .logResponses(true)
                .build()
        }

        ModelTypes.Google -> {
            val key = configuration?.apiKey
                ?: client.configurations.get<GoogleConfiguration>(GoogleConfiguration.KEY)?.apiKey
                ?: error("apiKey missing")
            GoogleAiGeminiChatModel.builder()
                .apiKey(key)
                .modelName(configuration?.modelName ?: name)
                .timeout(Duration.ofMinutes(30))
                .logRequestsAndResponses(true)
                .build()
        }

        ModelTypes.Ollama -> {
            val ollama = client.configurations.get<OllamaConfiguration>(OllamaConfiguration.KEY)
            val url = configuration?.baseUrl
                ?: ollama?.url
                ?: "http://localhost:11434/v1"
            val key = configuration?.apiKey
                ?: ollama?.apiKey
                ?: "ollama"
            val model = configuration?.baseUrl
                ?: ollama?.models?.get("default")
                ?: "llama3.3:70b-instruct-q8_0"
            OpenAiChatModel.builder()
                .baseUrl(url)
                .apiKey(key)
                .modelName(model)
                .timeout(Duration.ofMinutes(30))
                .logRequests(true)
                .logResponses(true)
                .build()
        }

        else -> throw UnsupportedOperationException("Unsupported model type: $type")
    }
}

interface ModelConfiguration {
    val temperature: Float
    val dimension: Long
}

@Serializable
@SerialName("default")
class DefaultConfiguration(
    override val temperature: Float = 0f,
    override val dimension: Long = 768
) : ModelConfiguration

@Serializable
class LanguageModelConfiguration(
    val baseUrl: String? = null,
    val apiKey: String? = null,
    val modelName: String? = null,
    override val temperature: Float = 0f,
    override val dimension: Long = 768
) : ModelConfiguration

val ModelConfigurationSerializers = SerializersModule {
    polymorphic(ModelConfiguration::class) {
        subclass(DefaultConfiguration::class, DefaultConfiguration.serializer())
        subclass(LanguageModelConfiguration::class, LanguageModelConfiguration.serializer())
    }
    polymorphicDefaultDeserializer(ModelConfiguration::class) { DefaultConfiguration.serializer() }
}