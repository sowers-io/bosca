package io.bosca.api

import com.apollographql.apollo.api.ApolloResponse
import com.apollographql.apollo.api.Operation

fun <T : Operation.Data> ApolloResponse<T>.validate() {
    if (hasErrors()) throw Exception(
        errors?.joinToString(separator = "\n") { it.message } ?: "Unknown error"
    )
    exception?.let { throw it }
}
