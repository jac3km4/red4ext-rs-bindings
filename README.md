# red4ext-rs-bindings

Automatically generated bindings for all RTTI types for [`red4ext-rs`](https://github.com/jac3km4/red4ext-rs).

## Usage

### Dependencies

Add the following to your `Cargo.toml`:

```toml
[dependencies]
red4ext-rs = { git = "https://github.com/jac3km4/red4ext-rs", features = ["log"], rev = "v0.10.0" }
red4ext-rs-bindings = { git = "https://github.com/jac3km4/red4ext-rs-bindings", rev = "v0.6.0" }
```

### Optional Features

- `mint`: Provides interoperability with the [`mint`](https://crates.io/crates/mint) crate for math types (e.g., Vector2).

## Examples

Use the bindings from your `red4ext-rs` plugin:

### Creating UI Elements

```rust
use red4ext_rs::{
    call, log, ScriptClassOps,
    types::{CName, Ref},
};
use red4ext_rs_bindings::{HdrColor, InkRectangleWidget, Vector2};

fn ui_example() -> Ref<InkRectangleWidget> {
    InkRectangleWidget::new_ref_with(|rect| {
        rect.size = Vector2 { x: 100.0, y: 100.0 };
        rect.tint_color = HdrColor {
            red: 1.,
            green: 0.,
            blue: 0.5,
            alpha: 1.,
        };
    })
    .unwrap()
}
```

### Interacting with the Player and Vehicles

```rust
use red4ext_rs::{
    call, log, ScriptClassOps,
    types::{CName, Ref},
};
use red4ext_rs_bindings::{PlayerPuppet, VehicleObject};

fn player_example(player: Ref<PlayerPuppet>) {
    // Call a method on the player
    let cost_result = call!(player, "GetHealingItemUseCost;" () -> i32);
    log::info!("GetHealingItemUseCost: {:?}", cost_result);

    // Get the player's mounted vehicle
    let vehicle = call!(player, "GetMountedVehicle;" () -> Ref<VehicleObject>).unwrap();
    let ps_name_result = call!(vehicle, "GetPSClassName;" () -> CName);

    log::info!("Vehicle PSClassName: {:?}", ps_name_result);
    if let Ok(name) = ps_name_result {
        log::info!("{}", name.as_str());
    }
}
```
