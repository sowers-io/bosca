package io.bosca.api

import com.apollographql.apollo.ApolloClient
import com.apollographql.apollo.api.*
import com.apollographql.apollo.api.http.HttpHeader
import com.apollographql.apollo.api.json.JsonReader
import com.apollographql.apollo.api.json.JsonWriter
import com.apollographql.apollo.interceptor.ApolloInterceptor
import com.apollographql.apollo.interceptor.ApolloInterceptorChain
import com.apollographql.apollo.network.http.DefaultHttpEngine
import com.apollographql.apollo.network.http.HttpEngine
import com.apollographql.apollo.network.ws.DefaultWebSocketEngine
import com.apollographql.apollo.network.ws.WebSocketConnection
import com.apollographql.apollo.network.ws.WebSocketEngine
import com.apollographql.apollo.network.ws.WebSocketNetworkTransport
import kotlinx.coroutines.flow.Flow
import okhttp3.OkHttpClient
import java.time.Duration
import java.time.LocalDateTime
import java.time.ZonedDateTime
import java.time.format.DateTimeFormatter

class NetworkClient {

    var graphqlToken: String? = null

    val graphql = ApolloClient.Builder()
        .serverUrl(System.getenv("BOSCA_GRAPHQL_URL") ?: "http://127.0.0.1:8000/graphql")
        .subscriptionNetworkTransport(
            WebSocketNetworkTransport.Builder()
                .serverUrl(System.getenv("BOSCA_WS_URL") ?: "ws://127.0.0.1:8000/ws")
                .reopenWhen { _, _ ->
                    true
                }
                .webSocketEngine(object : WebSocketEngine {
                    override suspend fun open(url: String, headers: List<HttpHeader>): WebSocketConnection {
                        graphqlToken?.let {
                            @Suppress("NAME_SHADOWING")
                            val headers = headers.toMutableList()
                            headers.add(HttpHeader("Authorization", "Bearer $it"))
                            return DefaultWebSocketEngine().open(url, headers)
                        } ?: println("missing graphql auth token")
                        return DefaultWebSocketEngine().open(url, headers)
                    }
                })
                .build()
        )
        .addCustomScalarAdapter(CustomScalarType("DateTime", ZonedDateTime::class.qualifiedName!!), object : Adapter<ZonedDateTime> {
            override fun fromJson(reader: JsonReader, customScalarAdapters: CustomScalarAdapters): ZonedDateTime {
                val response = reader.nextString() ?: error("missing date")
                return ZonedDateTime.parse(response)
            }
            override fun toJson(writer: JsonWriter, customScalarAdapters: CustomScalarAdapters, value: ZonedDateTime) {
                writer.value(value.format(DateTimeFormatter.ISO_OFFSET_DATE_TIME))
            }
        })
        .httpEngine(DefaultHttpEngine(600000))
        .addInterceptor(object : ApolloInterceptor {
            override fun <D : Operation.Data> intercept(
                request: ApolloRequest<D>,
                chain: ApolloInterceptorChain
            ): Flow<ApolloResponse<D>> {
                if (graphqlToken == null) return chain.proceed(request)
                return chain.proceed(request.newBuilder().addHttpHeader("Authorization", "Bearer $graphqlToken").build())
            }
        })
        .build()


    val http = OkHttpClient.Builder()
        .readTimeout(Duration.ofMinutes(10))
        .writeTimeout(Duration.ofMinutes(10))
        .connectTimeout(Duration.ofSeconds(10))
        .build()
}

object NetworkClientProvider {

    val client by lazy { NetworkClient() }
}