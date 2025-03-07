package io.bosca.util

import com.apollographql.apollo.api.Optional

fun <T: Any> T?.toOptional() = Optional.presentIfNotNull(this)

inline fun <reified T: Any> T.encodeToOptional() = encode<T>().toOptional()