package io.bosca.api

import io.bosca.graphql.AddCategoryMutation
import io.bosca.graphql.EditCategoryMutation
import io.bosca.graphql.GetCategoriesQuery
import io.bosca.graphql.fragment.Category
import io.bosca.graphql.type.CategoryInput

class ContentCategories(network: NetworkClient) : Api(network) {

    suspend fun getAll(): List<Category> {
        val response = network.graphql.query(GetCategoriesQuery()).execute()
        response.validate()
        return response.data?.content?.categories?.all?.map { it.category } ?: emptyList()
    }

    suspend fun add(category: CategoryInput): Category {
        val response = network.graphql.mutation(AddCategoryMutation(category)).execute()
        response.validate()
        return response.data?.content?.category?.add?.category ?: error("No category returned")
    }

    suspend fun edit(id: String, category: CategoryInput): Category {
        val response = network.graphql.mutation(EditCategoryMutation(id, category)).execute()
        response.validate()
        return response.data?.content?.category?.edit?.category ?: error("No category returned")
    }

    suspend fun delete(id: String) {
        TODO()
    }
}