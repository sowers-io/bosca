package io.bosca.workflow

interface ActivityRegistry {

    fun getActivity(id: String): Activity?
}