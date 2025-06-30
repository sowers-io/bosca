package io.bosca.api

import io.bosca.graphql.*
import io.bosca.graphql.fragment.Group
import io.bosca.graphql.fragment.LoginResponse
import kotlinx.coroutines.DelicateCoroutinesApi
import kotlinx.coroutines.GlobalScope
import kotlinx.coroutines.delay
import kotlinx.coroutines.launch
import kotlinx.coroutines.sync.Mutex
import kotlinx.coroutines.sync.withLock

class Security(network: NetworkClient) : Api(network) {

    private val mutex = Mutex()
    var keepTokenUpdated = true
    var token: LoginResponse? = null

    suspend fun refreshToken() {
        mutex.withLock {
            val response = network.graphql.mutation(RefreshTokenMutation(token?.refreshToken ?: return)).execute()
            response.validate()
            token = response.data?.security?.login?.refreshToken?.loginResponse
            network.graphqlToken = token?.token?.token
        }
    }

    private suspend fun loginInternal(identifier: String, password: String) {
        val response = network.graphql.mutation(LoginMutation(identifier, password)).execute()
        response.validate()
        token = response.data?.security?.login?.password?.loginResponse
        network.graphqlToken = token?.token?.token
    }

    @OptIn(DelicateCoroutinesApi::class)
    private fun keepTokenUpdated(identifier: String, password: String) {
        GlobalScope.launch {
            while (keepTokenUpdated) {
                val expiresAt = (token?.token?.expiresAt ?: 240).toLong() * 1000
                val issuedAt = (token?.token?.issuedAt ?: 0).toLong() * 1000
                val delay = (expiresAt - issuedAt) - 120_000
                delay(delay)
                if (!keepTokenUpdated) return@launch
                try {
                    refreshToken()
                } catch (e: Exception) {
                    println("failed to update token: $e")
                    if (!keepTokenUpdated) return@launch
                    try {
                        loginInternal(identifier, password)
                    } catch (e: Exception) {
                        println("failed to login, waiting 30 seconds... :: $e")
                        delay(30_000)
                    }
                }
            }
        }
    }

    suspend fun login(identifier: String, password: String) {
        loginInternal(identifier, password)
        keepTokenUpdated(identifier, password)
    }

    suspend fun addPrincipalGroup(principalId: String, groupId: String) {
        val response = network.graphql.mutation(AddPrincipalGroupMutation(groupId, principalId)).execute()
        response.validate()
    }

    suspend fun addGroup(name: String, description: String) {
        val response = network.graphql.mutation(AddGroupMutation(name, description)).execute()
        response.validate()
    }

    suspend fun getGroups(offset: Int, limit: Int): List<Group> {
        val response = network.graphql.query(GetGroupsQuery(offset, limit)).execute()
        response.validate()
        return response.data?.security?.groups?.all?.map { it.group } ?: emptyList()
    }

    suspend fun getPermissionActions(): List<String> {
        val response = network.graphql.query(GetPermissionActionsQuery()).execute()
        response.validate()
        return response.data?.security?.actions?.map { it.uppercase() } ?: emptyList()
    }
}