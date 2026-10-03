mod abstractFactory;
mod builder;
mod factoryMethod;
mod prototype;
mod singleton;

fn main() {
    abstractFactory::main();
    builder::main();
    factoryMethod::main();
    prototype::main();
    singleton::main();
}