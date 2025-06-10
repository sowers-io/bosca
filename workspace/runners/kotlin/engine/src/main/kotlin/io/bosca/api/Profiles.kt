package io.bosca.api

import io.bosca.graphql.AddProfileAttributeTypeMutation
import io.bosca.graphql.AddProfileAttributesMutation
import io.bosca.graphql.EditProfileAttributeTypeMutation
import io.bosca.graphql.GetProfileAttributeTypesQuery
import io.bosca.graphql.GetProfilesQuery
import io.bosca.graphql.fragment.Principal
import io.bosca.graphql.fragment.Profile
import io.bosca.graphql.fragment.ProfileAttributeType
import io.bosca.graphql.type.ProfileAttribute
import io.bosca.graphql.type.ProfileAttributeInput
import io.bosca.graphql.type.ProfileAttributeTypeInput

class Profiles(network: NetworkClient) : Api(network) {

    suspend fun getAll(offset: Int, limit: Int): List<Pair<Profile, Principal?>> {
        val response = network.graphql.query(GetProfilesQuery(offset, limit)).execute()
        response.validate()
        return response.data?.profiles?.all?.map { Pair(it.profile, it.principal?.principal) } ?: emptyList()
    }

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

    suspend fun addAttributes(id: String, attributes: List<ProfileAttributeInput>) {
        val response = network.graphql.mutation(AddProfileAttributesMutation(id, attributes)).execute()
        response.validate()
    }
}