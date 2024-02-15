# Structs, Enums, Generics, Impls and Traits

# Object Safe Traits
- ### A trait is meant to be object safe when the rust compiler have the ability to create trait objects for that trait
- ### For making a trait object safe the trait must follow these protocols:
  - All the methods in that trait must have `&self`, `&mut self` or `self` as an argument
  - The trait should not have `Self: Sized` as bound
  - All the trait methods should not have generic type parametres
  - All the methods in the trait should not return `Self` as return type
  - All the super traits must also be object safe
