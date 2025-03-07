package io.bosca.api

import io.bosca.graphql.AddProfileAttributeTypeMutation
import io.bosca.graphql.EditProfileAttributeTypeMutation
import io.bosca.graphql.GetProfileAttributeTypesQuery
import io.bosca.graphql.fragment.ProfileAttributeType
import io.bosca.graphql.type.ProfileAttributeTypeInput

class Profiles(network: NetworkClient) : Api(network) {

    suspend fun getAttributeTypes(): List<ProfileAttributeType> {
        val response = network.graphql.query(GetProfileAttributeTypesQuery()).execute()
        response.validate()
        return response.data?.profiles?.attributeTypes?.all?.map { it.profileAttributeType } ?: emptyList()
    }

    suspend fun addAttributeType(type: ProfileAttributeTypeInput) {
        val response = network.graphql.mutation(AddProfileAttributeTypeMutation(type)).execute()
        response.validate()
    }

    suspend fun editAttributeType(type: ProfileAttributeTypeInput) {
        val response = network.graphql.mutation(EditProfileAttributeTypeMutation(type)).execute()
        response.validate()
    }
}