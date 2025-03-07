package io.bosca.api

import io.bosca.graphql.*
import io.bosca.graphql.type.ConfigurationInput
import io.bosca.util.decode
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.launch
import java.util.concurrent.ConcurrentHashMap
import java.util.concurrent.atomic.AtomicBoolean

class Configurations(network: NetworkClient) : Api(network) {

    private val configurations = ConcurrentHashMap<String, GetConfigurationValueQuery.Configuration>()
    private val configurationIdKeys = ConcurrentHashMap<String, String>()

    private val listening = AtomicBoolean(false)

    private fun listen() {
        if (listening.getAndSet(true)) return
        @Suppress("OPT_IN_USAGE")
        GlobalScope.launch {
            network.graphql.subscription(OnConfigurationChangedSubscription()).toFlow().collect {
                clearIdFromCache(it.data?.configuration ?: return@collect)
            }
        }
    }

    private fun clearIdFromCache(id: String) {
        synchronized(configurationIdKeys) {
            val removeId = configurationIdKeys.remove(id) ?: return
            configurations.remove(removeId)
        }
    }

    suspend fun getRaw(key: String): Any? {
        listen()
        val response = network.graphql.query(GetConfigurationValueQuery(key)).execute()
        response.validate()
        response.data?.configurations?.configuration?.let {
            synchronized(configurationIdKeys) {
                configurationIdKeys[it.id] = it.key
                configurations[it.key] = it
            }
        }
        return configurations[key]?.value
    }

    suspend inline fun <reified T> get(key: String): T? {
        val raw = getRaw(key)
        return raw.decode<T>()
    }

    suspend fun set(configuration: ConfigurationInput) {
        val response = network.graphql.mutation(SetConfigurationMutation(configuration)).execute()
        response.validate()
        val id = response.data?.configurations?.setConfiguration?.id ?: return
        clearIdFromCache(id)
    }

    suspend fun delete(key: String) {
        val response = network.graphql.mutation(DeleteConfigurationMutation(key)).execute()
        response.validate()
        clearIdFromCache(response.data?.configurations?.deleteConfiguration ?: return)
    }
}