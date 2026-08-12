import com.elliancarlos.gasoline.core.Disabled
import com.elliancarlos.gasoline.core.ParameterGenerator
import com.elliancarlos.gasoline.core.ParametrizedTest
import com.elliancarlos.gasoline.core.Test
import com.elliancarlos.gasoline.core.assertions.Assert
import com.elliancarlos.gasoline.core.assertions.Either
import com.elliancarlos.gasoline.core.assertions.Option
import com.elliancarlos.gasoline.core.assertions.assertThat
import com.elliancarlos.gasoline.core.assertions.containsInOrder
import com.elliancarlos.gasoline.core.assertions.isEqualTo
import com.elliancarlos.gasoline.core.assertions.isFailure
import com.elliancarlos.gasoline.core.assertions.isLeft
import com.elliancarlos.gasoline.core.assertions.isNone
import com.elliancarlos.gasoline.core.assertions.isNotNull
import com.elliancarlos.gasoline.core.assertions.isNull
import com.elliancarlos.gasoline.core.assertions.isRight
import com.elliancarlos.gasoline.core.assertions.isSome
import com.elliancarlos.gasoline.core.assertions.isSuccess
import com.elliancarlos.gasoline.core.assertions.satisfies
import com.elliancarlos.gasoline.core.assertions.satisfiesFunctorLaws
import com.elliancarlos.gasoline.core.lifecycle.AfterAll
import com.elliancarlos.gasoline.core.lifecycle.AfterEach
import com.elliancarlos.gasoline.core.lifecycle.BeforeAll
import com.elliancarlos.gasoline.core.lifecycle.BeforeEach

class MyTests {
    @BeforeAll
    fun beforeAll() {
        println("BeforeAll")
    }

    @AfterAll
    fun afterAll() {
        println("AfterAll")
    }

    @BeforeEach
    fun beforeEach() {
        println("BeforeEach")
    }

    @AfterEach
    fun afterEach() {
        println("AfterEach")
    }

    @Test
    fun testAssertions() {
        val id: (Int) -> Int = { it }
        val f: (Int) -> String = { it.toString() }
        val g: (String) -> Int = { it.length }

        val assertSome = Assert<Option<Int>>(Option.Some(5))
        assertSome.satisfiesFunctorLaws(
            ida = id,
            f = f,
            g = g,
        )

        val assertNone = Assert<Option<Int>>(Option.None)
        assertNone.satisfiesFunctorLaws(
            ida = id,
            f = f,
            g = g,
        )

        Assert.Companion.assertThat(83).isNotNull()
        42.assertThat().isEqualTo(42)

        "Hello".assertThat().contains("ell")

        try {
            42.assertThat().contains("something")
        } catch (e: AssertionError) {
            println(e.message)
        }

        val list = listOf(1, 2, 3)
        list.assertThat().hasSize<List<Int>, Int>(3)
        list.assertThat().containsInOrder(2, 3)

        42
            .assertThat()
            .satisfies { it > 40 }

        data class RandomClass(
            val name: String? = null,
            val surname: String? = "anySurname",
        )

        RandomClass().name.assertThat().isNull()
        RandomClass().surname.assertThat().isNotNull()

        val rightValue: Either<Any, Int> = Either.Right(42)
        rightValue.assertThat().isRight()

        val leftValue: Either<Any, Int> = Either.Left("error")
        leftValue.assertThat().isLeft()

        val someValue: Option<String> = Option.Some("test")
        someValue.assertThat().isSome()

        Assert(Option.None as Option<Any>).isNone()

        val success = Result.success("test")
        success.assertThat().isSuccess()

        val error = RuntimeException("error")
        val failure = Result.failure<String>(error)
        failure.assertThat().isFailure()
    }

    @Test
    fun testAddition() {
        val result = 2 + 2
        result.assertThat().isEqualTo(4)
    }

    @Test
    fun testSubtraction() {
        val result = 5 - 3
        result.assertThat().isEqualTo(2)
    }

    @Disabled
    @Test
    fun testFailingTest() {
        val result = 2 * 2
        result.assertThat().isEqualTo(5)
    }

    class IntGenerator : ParameterGenerator<Int> {
        override fun generate(): Sequence<Int> = listOf(0, 2, 4, 222).asSequence()
    }

    @ParametrizedTest(IntGenerator::class)
    fun testParametrizedTest(number: Int) {
        number.assertThat().satisfies { it % 2 == 0 }
    }
}
