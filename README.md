# firmament
A Rust layout algorithm based on Flutter/Druid/Iced.


## How does it work?


## Resources
https://flutter.dev/docs/development/ui/layout
https://flutter.dev/docs/codelabs/layout-basics
```rust
impl MainWidget {
    fn build(context: BuildContext) -> Widget {
        Row(
            children: [
                BlueBox(),
                BlueBox(),
                BlueBox(),
            ]
        )
    }
}

impl BlueBox {
    fn build(context: BuildContext) -> Widget {
        Container {
            width: 50.0,
            height: 50.0,
            decoration: BoxDecoration(
                color: ...,
                border: ...
            )
        }
    }
}
```

# Axis size and alignment
`Row` has a horizontal main axis and `Column` vertical main axis .
`mainAxisSize` determines how much space `Row` an `Column` can occupy on their main axes.

`MainAxis.max`
`MainAxis.min`

<!-- ## iced
In Iced, `Widget.layout` takes an argument of renderer which is necesary for widgets like svg or text related things.  -->