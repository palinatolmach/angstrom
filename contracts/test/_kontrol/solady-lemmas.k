requires "evm.md"
requires "foundry.md"

module SOLADY-LEMMAS
    imports BOOL
    imports FOUNDRY
    imports INFINITE-GAS
    imports INT-SYMBOLIC
    imports MAP-SYMBOLIC
    imports SET-SYMBOLIC

    syntax StepSort ::= Int
                      | Bool
                      | Bytes
                      | Set
 // -------------------------

    syntax KItem ::= runLemma ( StepSort )
                   | doneLemma( StepSort )

    rule <k> runLemma(T) => doneLemma(T) ... </k>

    //
    // Bool
    //

    rule bool2Word ( X ) => 1 requires X         [simplification]
    rule bool2Word ( X ) => 0 requires notBool X [simplification]

    rule chop ( bool2Word ( X ) <<Int Y ) => bool2Word ( X ) <<Int Y requires Y <Int 256 [simplification]

    rule chop ( 1 <<Int Y ) => 1 <<Int Y requires Y <Int 256 [simplification]

    rule ( A:Int >>Int B:Int ) >>Int C:Int => A >>Int ( B +Int C ) [simplification]

    rule A =/=Int B => notBool ( A ==Int B )
      [simplification, comm]

    rule #Ceil ( byte (X:Int, Y:Int) ) => #Top
      requires 0 <=Int X andBool X <Int 32
       andBool 0 <=Int Y andBool Y <Int pow256
       [simplification]

    //
    // Bitwise
    //

    rule A >>Int B ==Int 0 => log2Int ( A ) <=Int B
      [simplification, comm, concrete(A)]

    rule 0 <=Int A >>Int B => true
      requires 0 <=Int A andBool 0 <=Int B
      [simplification]

    rule A >>Int B <=Int C => A <=Int ( C <<Int B ) +Int ( 2 ^Int B ) -Int 1
      requires C +Int 1 ==Int 2 ^Int ( log2Int ( C +Int 1 ) )
      [simplification, concrete(B, C)]

    rule C <Int A >>Int B => ( C <<Int B ) +Int ( 2 ^Int B ) -Int 1 <Int A
    requires C +Int 1 ==Int 2 ^Int ( log2Int ( C +Int 1 ) )
      [simplification, concrete(B, C)]

    rule X xorInt maxUInt256 => maxUInt256 -Int X
      requires #rangeUInt ( 256 , X )
      [simplification]

    //
    // Comparisons
    //

    // Concrete to the left of comparisons, prioritising <=Int and <Int
    rule X >=Int Y => Y <=Int X [simplification, concrete(Y)]
    rule X  >Int Y => Y  <Int X [simplification, concrete(Y)]

    // Negations at K-level
    rule notBool X <=Int Y => Y  <Int X [simplification]
    rule notBool X  <Int Y => Y <=Int X [simplification]
    rule notBool X  >Int Y => X <=Int Y [simplification]
    rule notBool X >=Int Y => X  <Int Y [simplification]

    // Negations ad ML-level
    rule { true #Equals ( notBool X <=Int Y ) } => { true #Equals Y  <Int X } [simplification]
    rule { true #Equals ( notBool X  <Int Y ) } => { true #Equals Y <=Int X } [simplification]
    rule { true #Equals ( notBool X  >Int Y ) } => { true #Equals X <=Int Y } [simplification]
    rule { true #Equals ( notBool X >=Int Y ) } => { true #Equals X  <Int Y } [simplification]
    rule { ( notBool X <=Int Y ) #Equals true } => { true #Equals Y  <Int X } [simplification]
    rule { ( notBool X  <Int Y ) #Equals true } => { true #Equals Y <=Int X } [simplification]
    rule { ( notBool X  >Int Y ) #Equals true } => { true #Equals X <=Int Y } [simplification]
    rule { ( notBool X >=Int Y ) #Equals true } => { true #Equals X  <Int Y } [simplification]

    //
    // Arithmetic
    //

    rule A modInt B => A
      requires 0 <=Int A andBool A <Int B
      [simplification]

    rule B *Int A <Int C => A <Int C /Int B
      requires C modInt B ==Int 0
      [simplification, concrete(B, C)]

    rule A <=Int C *Int B => A /Int C <=Int B
      requires A modInt C ==Int 0
      [simplification, concrete(A, C)]

    rule A <=Int B -Int C => C <=Int B -Int A
      [simplification, concrete(A, B)]
endmodule