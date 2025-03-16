package io.bosca.workflow.json

import com.dashjoin.jsonata.Jsonata
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.graphql.type.ActivityParameterInput
import io.bosca.graphql.type.ActivityParameterType
import io.bosca.util.*
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.coroutines.Dispatchers
import kotlinx.coroutines.withContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.builtins.serializer
import java.text.SimpleDateFormat
import java.time.OffsetDateTime
import java.time.format.DateTimeParseException
import kotlin.math.exp

@Serializable
data class JSONataConfiguration(
    val expression: String
)

class JSONata(client: Client) : Activity(client) {

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "JSONata",
            description = "Execute JSONata Expression",
            inputs = listOf(ActivityParameterInput(INPUT_NAME, ActivityParameterType.SUPPLEMENTARY)),
            outputs = listOf(ActivityParameterInput(OUTPUT_NAME, ActivityParameterType.SUPPLEMENTARY)),
        )
    }

    private fun parseDate(input: Any?): Long {
        if (input == null) return 0L
        return when (input) {
            is String -> {
                try {
                    OffsetDateTime.parse(input).toInstant().toEpochMilli()
                } catch (e: DateTimeParseException) {
                    SimpleDateFormat("mm/dd/yyyy hh:mm a").parse(input).time
                }
            }

            is Long -> {
                input.toLong()
            }

            is Float -> {
                val seconds = input.toLong()
                val milliseconds = ((input - seconds) * 1000).toLong()
                return seconds * 1000 + milliseconds
            }

            else -> {
                throw UnsupportedOperationException("Unsupported date type: ${input::class.simpleName} -> $input")
            }
        }
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val configuration = getConfiguration<JSONataConfiguration>(job)
        val file = if (hasInputs(job)) getInputSupplementaryFile(context, job, INPUT_NAME) else getContentFile(context, job)
        val data = withContext(Dispatchers.IO) { file.readText().parseToJsonElement().toAny() }
        val expression = Jsonata.jsonata(configuration.expression)
        expression.registerFunction("parseDate", ::parseDate)
        val result = expression.evaluate(data).toJsonElement()
        setSupplementaryContents(
            job,
            OUTPUT_NAME,
            "JSONata Output",
            String.serializer(),
            result.toString()
        )
    }

    companion object {
        const val ID = "data.jsonata"
        const val INPUT_NAME = "supplementary"
        const val OUTPUT_NAME = "supplementary"
    }
}