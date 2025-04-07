package io.bosca.workflow.email

import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.decode
import io.bosca.util.json
import io.bosca.util.toJsonElement
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.SerialName
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.contentOrNull
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
import okhttp3.MediaType.Companion.toMediaType
import okhttp3.OkHttpClient
import okhttp3.Request
import okhttp3.RequestBody.Companion.toRequestBody
import org.thymeleaf.TemplateEngine
import org.thymeleaf.context.Context
import org.thymeleaf.templateresolver.StringTemplateResolver
import java.util.*

@Serializable
data class EmailConfiguration(
    val slug: String,
    val attributes: Map<String, String> = emptyMap()
)

@Serializable
data class TemplateProfile(
    val email: String,
    val name: String
)

@Serializable
data class TemplateAttributes(
    val profile: TemplateProfile,
    val subject: String,
    val locale: String,
    val html: String,
    val text: String
)

@Serializable
data class SendGridConfiguration(
    val token: String?,
    val from: SendGridEmail,
)

@Serializable
data class SendGridEmail(
    val email: String,
    val name: String
)

@Serializable
data class Personalization(
    val subject: String,
    val to: List<SendGridEmail>
)

@Serializable
data class SendGridContent(val type: String, val value: String)

@Serializable
data class SendGridRequest(
    val from: SendGridEmail,
    val subject: String,
    val content: List<SendGridContent>,
    @SerialName("personalizations")
    val personalization: List<Personalization>
)

class EmailActivity(client: Client) : Activity(client) {

    private val api = OkHttpClient.Builder().build()

    override val id: String = ID

    override suspend fun toActivityDefinition(): ActivityInput {
        return ActivityInput(
            id = id,
            name = "Send Email",
            description = "Send an email",
            configuration = emptyMap<String, Any>().toOptional(),
            inputs = emptyList(),
            outputs = emptyList()
        )
    }

    private fun newEngine(type: String) = TemplateEngine().apply {
        setTemplateResolver(StringTemplateResolver().apply {
            setTemplateMode(type)
        })
    }

    override suspend fun execute(context: ActivityContext, job: WorkflowJob) {
        val sendGridConfiguration = client.configurations.get<SendGridConfiguration>("sendgrid")
        val profile = job.profile?.profile ?: error("profile missing")
        val configuration = getConfiguration<EmailConfiguration>(job)
        val template = client.metadata.getBySlug(configuration.slug) ?: error("template not found")
        val attributes = template.attributes?.decode<TemplateAttributes>() ?: error("template attributes not found")
        val ctx = Context(Locale.of(attributes.locale)).apply {
            configuration.attributes.forEach { (key, value) -> setVariable(key, value) }
        }
        val email =
            profile.attributes.firstOrNull { it.typeId == attributes.profile.email }?.attributes?.toJsonElement()?.jsonObject?.get(
                "email"
            )?.jsonPrimitive?.contentOrNull ?: error("email missing")
        val name =
            profile.attributes.firstOrNull { it.typeId == attributes.profile.name }?.attributes?.toJsonElement()?.jsonObject?.get(
                "name"
            )?.jsonPrimitive?.contentOrNull ?: error("name missing")
        val html = newEngine("HTML").process(attributes.html, ctx)
        val text = newEngine("TEXT").process(attributes.text, ctx)
        if (sendGridConfiguration?.token != null) {
            send(sendGridConfiguration, attributes.subject, name, email, html, text)
        } else {
            TODO()
        }
    }

    private fun send(
        cfg: SendGridConfiguration,
        subject: String,
        name: String,
        email: String,
        html: String,
        text: String
    ) {
        val request = Request.Builder().apply {
            url("https://api.sendgrid.com/v3/mail/send")
            post(
                json.encodeToString(
                    SendGridRequest(
                        from = cfg.from,
                        subject = subject,
                        personalization = listOf(Personalization(subject, listOf(SendGridEmail(email, name)))),
                        content = listOf(
                            SendGridContent("text/plain", text),
                            SendGridContent("text/html", html)
                        )
                    )
                ).toRequestBody("application/json".toMediaType())
            )
            addHeader("Authorization", "Bearer ${cfg.token}")
            addHeader("Content-Type", "application/json")
            addHeader("Accept", "application/json")
        }
        val response = api.newCall(request.build()).execute()
        if (!response.isSuccessful) {
            throw Exception("SendGrid error: ${response.body?.string()}")
        }
    }

    companion object {
        const val ID = "email.send"
    }
}