@file:Suppress("EXTENSION_SHADOWED_BY_MEMBER")

package com.elliancarlos.gasoline.core.assertions

@JvmName("CollectionHasSize")
inline fun <reified T> Assert<Collection<T>>.hasSize(expected: Int): Assert<Collection<T>> =
    apply {
        if (actual.size != expected) {
            throw AssertionException("Expected size $expected but was ${actual.size}")
        }
    }

@JvmName("ListHasSize")
inline fun <reified T> Assert<List<T>>.hasSize(expected: Int): Assert<List<T>> =
    apply {
        if (actual.size != expected) {
            throw AssertionException("Expected size $expected but was ${actual.size}")
        }
    }

@JvmName("SetHasSize")
inline fun <reified T> Assert<Set<T>>.hasSize(expected: Int): Assert<Set<T>> =
    apply {
        if (actual.size != expected) {
            throw AssertionException("Expected size $expected but was ${actual.size}")
        }
    }

fun <T> Assert<Sequence<T>>.all(predicate: (T) -> Boolean): Assert<Sequence<T>> =
    apply {
        if (!actual.all(predicate)) {
            throw AssertionException("Not all elements satisfied the predicate")
        }
    }

fun <T> Assert<Sequence<T>>.any(predicate: (T) -> Boolean): Assert<Sequence<T>> =
    apply {
        if (!actual.any(predicate)) {
            throw AssertionException("No elements satisfied the predicate")
        }
    }

fun <T> Assert<List<T>>.containsInOrder(vararg elements: T): Assert<List<T>> =
    apply {
        val actualList = actual
        val expectedList = elements.toList()

        if (!actualList.windowed(expectedList.size).any { it == expectedList }) {
            throw AssertionException("Expected elements $expectedList in order, but was $actualList")
        }
    }

fun <T : Comparable<T>> Assert<List<T>>.isOrdered(): Assert<List<T>> =
    apply {
        if (!actual.zipWithNext().all { (a, b) -> a <= b }) {
            throw AssertionException("List is not ordered")
        }
    }

fun <T> Assert<List<T>>.isOrdered(comparator: Comparator<T>): Assert<List<T>> =
    apply {
        if (!actual.zipWithNext().all { (a, b) -> comparator.compare(a, b) <= 0 }) {
            throw AssertionException("List is not ordered according to the provided comparator")
        }
    }
