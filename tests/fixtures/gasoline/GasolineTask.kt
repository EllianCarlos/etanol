package com.elliancarlos.gasoline

import com.elliancarlos.gasoline.core.TestReporter
import com.elliancarlos.gasoline.core.TestRunner
import com.elliancarlos.gasoline.core.validator.isParametrized
import com.elliancarlos.gasoline.core.validator.isTest
import org.gradle.api.DefaultTask
import org.gradle.api.GradleException
import org.gradle.api.plugins.JavaPluginExtension
import org.gradle.api.tasks.SourceSet
import org.gradle.api.tasks.TaskAction
import java.net.URLClassLoader
import kotlin.reflect.full.createInstance
import kotlin.reflect.full.functions

open class GasolineTask : DefaultTask() {

    @TaskAction
    fun runGasolineTests() {
        val sourceSets = project.extensions.getByType(JavaPluginExtension::class.java).sourceSets
        val testSourceSet = sourceSets.getByName(SourceSet.TEST_SOURCE_SET_NAME)
        val classPathUrls = (testSourceSet.runtimeClasspath.files + testSourceSet.output.classesDirs.files).map { it.toURI().toURL() }.toTypedArray()
        val classLoader = URLClassLoader(classPathUrls, this.javaClass.classLoader)

        logger.lifecycle("Searching for Gasoline tests...")

        val testClasses = testSourceSet.output.classesDirs.files.flatMap { dir ->
            dir.walkTopDown().filter { it.isFile && it.name.endsWith(".class") }
                .map { file ->
                    val className = file.relativeTo(dir).path.replace('/', '.').removeSuffix(".class")
                    runCatching { classLoader.loadClass(className) }.getOrNull()
                }
                .filterNotNull()
        }

        val summaries = testClasses.mapNotNull { clazz ->
            val kClass = clazz.kotlin
            if (kClass.isAbstract || kClass.isCompanion) return@mapNotNull null

            val hasTests = kClass.functions.any { it.isTest() || it.isParametrized() }

            if (hasTests) {
                try {
                    logger.lifecycle("Running tests in ${kClass.simpleName}...")
                    val instance = kClass.createInstance()
                    val summary = TestRunner.runTests(instance)
                    TestReporter.generateReport(summary).also(::println)
                    summary
                } catch (e: Exception) {
                    logger.error("Failed to instantiate or run tests for ${kClass.simpleName}", e)
                    null
                }
            } else {
                null
            }
        }

        if (summaries.isEmpty()) {
            logger.lifecycle("No Gasoline tests found.")
            return
        }

        val total = summaries.sumOf { it.totalTests }
        val succeeded = summaries.sumOf { it.passedTests }
        val failed = summaries.sumOf { it.failedTests }
        val failedTests = summaries.filter { it.failedTests > 0 }

        logger.lifecycle("------------------------------------------------")
        logger.lifecycle("Gasoline Test Run Finished")
        logger.lifecycle("Total: $total, Succeeded: $succeeded, Failed: $failed")
        logger.lifecycle("------------------------------------------------")

        if (failedTests.isNotEmpty()) {
            throw GradleException("$failed test(s) failed.")
        }
    }
}
