package com.elliancarlos.gasoline.core

import com.elliancarlos.gasoline.core.lifecycle.AfterAll
import com.elliancarlos.gasoline.core.lifecycle.AfterEach
import com.elliancarlos.gasoline.core.lifecycle.BeforeAll
import com.elliancarlos.gasoline.core.lifecycle.BeforeEach
import com.elliancarlos.gasoline.core.validator.isDisabled
import com.elliancarlos.gasoline.core.validator.isParametrized
import com.elliancarlos.gasoline.core.validator.isTest
import kotlin.reflect.KFunction
import kotlin.reflect.KParameter
import kotlin.reflect.full.createInstance
import kotlin.reflect.full.findAnnotation
import kotlin.reflect.full.functions
import kotlin.sequences.flatMap

object TestRunner {
    private data class TestExecutable(
        val method: KFunction<*>,
        val instance: Any,
        val args: Array<Any?> = emptyArray(),
    ) {
        fun execute() {
            method.call(instance, *args)
        }
    }

    private fun preSuite(testClass: Any) =
        this.also {
            findFirstMethodAnnotatedAndExecute<BeforeAll>(testClass)
        }

    private fun postSuite(testClass: Any) =
        this.also {
            findFirstMethodAnnotatedAndExecute<AfterAll>(testClass)
        }

    private fun preTest(testClass: Any) =
        this.also {
            findFirstMethodAnnotatedAndExecute<BeforeEach>(testClass)
        }

    private fun postTest(testClass: Any) =
        this.also {
            findFirstMethodAnnotatedAndExecute<AfterEach>(testClass)
        }

    fun runTests(testClass: Any): TestSummary {
        preSuite(testClass)
        val testResults =
            testClass::class
                .functions
                .asSequence()
                .filter { it.isDisabled().not() }
                .filter { it.isTest() }
                .flatMap { method -> unfoldAndExecute(testClass, method) }
                .toSummary(testClass::class.simpleName ?: "Unknown")
        postSuite(testClass)
        return testResults
    }

    private fun unfoldAndExecute(
        testClass: Any,
        method: KFunction<*>,
    ) = unfoldParametrizedTests(testClass, method).map { testExecutable ->
        preTest(testClass)
        executeTest(testExecutable)
            .also { postTest(testClass) }
    }

    private fun executeTest(testExecutable: TestExecutable) =
        try {
            testExecutable.execute()
            TestResult.Success(testExecutable.method.name)
        } catch (e: Exception) {
            TestResult.Failure(
                testName = testExecutable.method.name,
                error =
                    (e.cause?.message ?: e.message ?: "Unknown error").plus(" at \n" + e.stackTrace?.joinToString("\n")),
            )
        }

    private fun extractParameters(method: KFunction<*>) =
        if (method.isParametrized()) {
            getGenerator(method)?.generate()
        } else {
            null
        }

    private fun getGenerator(method: KFunction<*>) =
        (method.findAnnotation<ParametrizedTest>())
            ?.generator
            ?.createInstance()

    private fun executeMethod(
        instance: Any,
        method: KFunction<*>,
    ) {
        if (method.parameters.any { it.kind == KParameter.Kind.INSTANCE }) {
            method.call(instance)
        } else {
            method.call()
        }
    }

    private fun unfoldParametrizedTests(
        instance: Any,
        method: KFunction<*>,
    ): Sequence<TestExecutable> {
        if (method.isParametrized().not()) {
            return sequence { yield(TestExecutable(method, instance)) }
        }

        return extractParameters(method)
            ?.map { parameter -> TestExecutable(method, instance, arrayOf(parameter)) }
            ?: sequence {}
    }

    private inline fun <
        reified predicatedAnnotation : Any,
    > findFirstMethodAnnotated(testClass: Any) =
        testClass::class
            .functions
            .firstOrNull { it.annotations.firstOrNull { annotation -> annotation is predicatedAnnotation } != null }

    private inline fun <
        reified predicatedAnnotation : Any,
    > findFirstMethodAnnotatedAndExecute(testClass: Any) =
        findFirstMethodAnnotated<predicatedAnnotation>(testClass)?.let { executeMethod(testClass, it) }
}
