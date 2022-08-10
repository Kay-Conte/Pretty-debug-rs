# **Pretty Debug**

Pretty Debug adds a `Debugger` trait which simply allows you to print to the console with user defined module tags.

This crate is fine to use _though I doubt will ever be needed due to severe lack of functionality_ but is intended
as a test to publish to crates.io

## **Intended Usage**

Simply define an enum implementing the Debugger trait as following:

```rust
enum Debug {
    Module,
}

impl Debugger for Debug {
    fn display(&self) -> String {
        match self {
            Debug::Module => "[From Module]",
        }
        .to_string()
    }
}
```

Note: the return of the `display` function are used as a prefix.

Then call the debug method on the desired variant of your enum:

```rust
Debug::Module.debug("This was sent from the Module module")
```

## Additional Notes

You may refer to the provided example for additional ergonomics.

I expect this crate to only be in use by me, though feel free to make any requests through github with additional features or changes.

I may add some other smaller features in the future if desired for my projects, such as a suffix and default function definitions later.

<sub>I also may have made 3 publishes trying to figure out why crates.io wasn't coloring my code</sub>
