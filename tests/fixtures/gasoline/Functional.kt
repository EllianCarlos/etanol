package com.elliancarlos.gasoline.core.assertions

sealed class Either<out L, out R> {
    data class Left<L>(
        val value: L,
    ) : Either<L, Nothing>()

    data class Right<R>(
        val value: R,
    ) : Either<Nothing, R>()
}

inline fun <reified L, reified R> Assert<Either<L, R>>.isLeft(): Assert<Either<L, R>> =
    apply {
        when (actual) {
            is Either.Left -> Assert(actual.value)
            is Either.Right -> throw AssertionException("Expected Left but was Right(${actual.value})")
        }
    }

inline fun <reified L, reified R> Assert<Either<L, R>>.isRight(): Assert<Either<L, R>> =
    apply {
        when (actual) {
            is Either.Left -> throw AssertionException("Expected Right but was Left(${actual.value})")
            is Either.Right -> Assert(actual.value)
        }
    }

@JvmName("EitherLeftValue")
fun <L> Assert<Either.Left<L>>.value(): Assert<L> = Assert(actual.value)

@JvmName("EitherRightValue")
fun <R> Assert<Either.Right<R>>.value(): Assert<R> = Assert(actual.value)

sealed class Option<out T> {
    object None : Option<Nothing>()

    data class Some<T>(
        val value: T,
    ) : Option<T>()
}

inline fun <reified T> Assert<Option<T>>.isSome(): Assert<Option<T>> =
    apply {
        when (val optionValue = actual) {
            is Option.Some -> Assert(optionValue)
            is Option.None -> throw AssertionException("Expected Some but was None")
        }
    }

inline fun <reified T> Assert<Option<T>>.isNone(): Assert<Option<T>> =
    apply {
        when (val optionValue = actual) {
            is Option.Some -> throw AssertionException("Expected None but was Some(${optionValue.value})")
            is Option.None -> Assert(optionValue)
        }
    }

@JvmName("OptionSomeValue")
inline fun <reified T> Assert<Option.Some<T>>.value(): Assert<T> = Assert(actual.value)

@Suppress("UNCHECKED_CAST")
inline fun <reified T> Assert<Result<T>>.isSuccess(): Assert<T> =
    apply {
        actual.fold(
            onSuccess = { Assert(it) },
            onFailure = { throw AssertionException("Expected Success but was Failure(${it.message})") },
        )
    } as Assert<T>

@Suppress("UNCHECKED_CAST")
inline fun <reified T> Assert<Result<T>>.isFailure(): Assert<Throwable> =
    apply {
        actual.fold(
            onSuccess = { throw AssertionException("Expected Failure but was Success($it)") },
            onFailure = { Assert(it) },
        )
    } as Assert<Throwable>
