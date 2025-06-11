package io.bosca.workflow

import io.bosca.api.Client

interface ActivityRegistry {

    fun getActivity(id: String): Activity?

    suspend fun install(client: Client)

    companion object {
        lateinit var instance: ActivityRegistry
    }
}
