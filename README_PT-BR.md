[ENGLISH](README.md)

Esta é uma biblioteca fictícia utilizada como caso de estudo.

Este crate fornece uma macro procedural `FromLine` para derivar a implementação do trait `from_line::traits::from_string::FromString` para uma struct. A trait `FromLine` permite que você converta facilmente uma linha de texto em uma instância de estrutura, analisando os campos da linha.

# Uso

Para usar este crate, você precisa adicionar a dependência from_line_derive ao seu arquivo Cargo.toml:

```toml
[dependencies]
from_line_derive = "0.1"
```

Então, você pode usar a macro `FromLine` na sua estrutura:

```rust
use from_line_derive::FromLine;

#[derive(FromLine)]
struct Person {
    #[from_line(0..5)]
    name: String,

    #[from_line(6..10)]
    age: u32,

    #[from_line(11..)]
    address: String,
}
```

O atributo `#[from_line(X..Y)]` especifica o intervalo de caracteres na linha que deve ser usado para popular o campo correspondente. O intervalo inclui o índice inicial `X` e exclui o índice final `Y`.

Finalmente, você pode usar o método from_line para criar uma instância da estrutura a partir de uma linha de texto:

```rust
let line = "John  25  123 Main St";
let person = Person::from_line(line);

assert_eq!(person.name, "John");
assert_eq!(person.age, 25);
assert_eq!(person.address, "123 Main St");
```

Você também pode criar sua implementação personalizada a um tipo através da trait FromString caso algum tipo não seja compatível com o macro.
