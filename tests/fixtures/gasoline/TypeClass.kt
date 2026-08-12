package com.elliancarlos.gasoline.core.assertions

fun <A, B, C> Assert<Option<A>>.satisfiesFunctorLaws(
    ida: (A) -> A,
    f: (A) -> B,
    g: (B) -> C,
): Assert<Option<A>> =
    apply {
        // Identity law: map(id) == id
        val mappedIdentity: Option<A> = actual.map(ida)

        if (mappedIdentity != actual) {
            throw AssertionException("Functor identity law violated: Expected $actual but got $mappedIdentity")
        }

        if (mappedIdentity != actual) {
            throw AssertionException("Functor identity law violated: Expected $actual but got $mappedIdentity")
        }

        // Composition law: map(f compose g) == map(f) compose map(g)
        val composed: Option<C> = actual.map(f).map(g)
        val separately: Option<C> = actual.map { a -> g(f(a)) }

        if (composed != separately) {
            throw AssertionException(
                "Functor composition law violated: \n" +
                    "Composed: $composed\n" +
                    "Separately: $separately",
            )
        }
    }

object OptionFunctor {
    fun <A, B> map(
        option: Option<A>,
        f: (A) -> B,
    ): Option<B> =
        when (option) {
            is Option.Some -> Option.Some(f(option.value))
            is Option.None -> Option.None
        }
}

// Extension function for Option mapping
fun <A, B> Option<A>.map(f: (A) -> B): Option<B> = OptionFunctor.map(this, f)
