package io.bosca.workflow.email

import com.sendgrid.Method
import com.sendgrid.Request
import com.sendgrid.SendGrid
import com.sendgrid.helpers.mail.Mail
import com.sendgrid.helpers.mail.objects.*
import io.bosca.api.Client
import io.bosca.graphql.fragment.WorkflowJob
import io.bosca.graphql.type.ActivityInput
import io.bosca.util.decode
import io.bosca.util.toJsonElement
import io.bosca.util.toOptional
import io.bosca.workflow.Activity
import io.bosca.workflow.ActivityContext
import kotlinx.serialization.Serializable
import kotlinx.serialization.json.contentOrNull
import kotlinx.serialization.json.jsonObject
import kotlinx.serialization.json.jsonPrimitive
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
data class SendGridFrom(val email: String, val name: String)

@Serializable
data class SendGridConfiguration(
    val token: String?,
    val from: SendGridFrom,
)

class EmailActivity(client: Client) : Activity(client) {

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

    private fun send(cfg: SendGridConfiguration, subject: String, name: String, email: String, html: String, text: String) {
        val client = SendGrid(cfg.token)
        val response = client.api(Request().apply {
            endpoint = "mail/send"
            method = Method.POST
            body = Mail().apply {
                setSubject(subject)
                setFrom(Email(cfg.from.email, cfg.from.name))
                    setTrackingSettings(TrackingSettings().apply {
                        clickTrackingSetting = ClickTrackingSetting().apply {
                            enable = false
                        }
                    })
                addPersonalization(Personalization().apply {
                    addTo(Email(email, name))
                })
                addContent(Content("text/plain", text))
                addContent(Content("text/html", html))
            }.build()
        })
        if (response.statusCode >= 400) {
            throw Exception("SendGrid error: ${response.body}")
        }
    }

    companion object {
        const val ID = "email.send"
    }
}