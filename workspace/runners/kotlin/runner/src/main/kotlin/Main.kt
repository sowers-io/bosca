package io.bosca

import com.github.ajalt.clikt.command.main
import com.github.ajalt.clikt.core.subcommands
import io.bosca.commands.workflows.Enqueue
import io.bosca.commands.Install
import io.bosca.commands.Run
import io.bosca.commands.Runner
import io.bosca.commands.content.Content
import io.bosca.commands.content.Search
import io.bosca.commands.workflows.Workflows
import io.bosca.documents.DocumentSerializers
import io.bosca.util.json
import io.bosca.workflow.ai.schema.JsonSchemaSerializers
import io.bosca.workflow.ext.ModelConfigurationSerializers
import kotlinx.serialization.ExperimentalSerializationApi
import kotlinx.serialization.json.ClassDiscriminatorMode
import kotlinx.serialization.json.Json
import kotlinx.serialization.modules.plus

@OptIn(ExperimentalSerializationApi::class)
suspend fun main(args: Array<String>) {
    json = Json {
        serializersModule = ModelConfigurationSerializers + DocumentSerializers + JsonSchemaSerializers
        classDiscriminator = "type"
        classDiscriminatorMode = ClassDiscriminatorMode.POLYMORPHIC
        ignoreUnknownKeys = true
    }

    Runner().subcommands(
        Install(),
        Run(),
        Workflows().subcommands(Enqueue()),
        Content().subcommands(Search())
    ).main(args)

    System.out.flush()
    System.err.flush()
}
