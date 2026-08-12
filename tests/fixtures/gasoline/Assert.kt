package com.elliancarlos.gasoline.core.assertions

data class Assert<T>(
    val actual: T,
) {
    companion object {
        fun <T> assertThat(actual: T): Assert<T> = Assert(actual)
    }

    fun apply(block: Assert<T>.() -> Unit): Assert<T> {
        block()
        return this
    }

    inline fun <reified C : Collection<E>, reified E> hasSize(expected: Int): Assert<C> {
        if (actual !is C) {
            throw AssertionError("Expected a Collection of ${E::class.simpleName} but was ${actual!!::class.simpleName}")
        }

        if (actual.size != expected) {
            throw AssertionError("Expected size $expected but was ${actual.size}")
        }
        return this as Assert<C>
    }

    infix fun contains(substring: String): Assert<T> {
        if (actual !is String) {
            throw AssertionError("Expected a String but was ${actual!!::class.simpleName}")
        }

        if (!actual.contains(substring)) {
            throw AssertionError("Expected '$actual' to contain '$substring'")
        }
        return this
    }
}

inline fun <reified T> T.assertThat(): Assert<T> = Assert(this)

class AssertionException(
    message: String,
) : AssertionError(message)
