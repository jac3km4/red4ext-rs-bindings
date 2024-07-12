# red4rs-bindings
Automatically generated for all RTTI types for [red4rs](https://github.com/jac3km4/red4rs)

# usage
- add the dependencies
  ```toml
  [dependencies]
  red4rs = { git = "https://github.com/jac3km4/red4rs", features = ["log",], rev = "v0.1.7" }
  red4rs-bindings = { git = "https://github.com/jac3km4/red4rs-bindings", rev = "v0.1.8" }
  ```
- use them from your red4rs plugin
    ```rust
    use red4rs::{
        call, log,
        types::{CName, Ref, ScriptClassOps},
    };
    use red4rs_bindings::{HdrColor, InkRectangleWidget, PlayerPuppet, Vector2, VehicleObject};

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

    fn player_example(player: Ref<PlayerPuppet>) {
        let cost_result = call!(player, "GetHealingItemUseCost;" () -> i32);
        log::info!("GetHealingItemUseCost: {:?}", cost_result);

        let vehicle = call!(player, "GetMountedVehicle;" () -> Ref<VehicleObject>).unwrap();
        let ps_name_result = call!(vehicle, "GetPSClassName;" () -> CName);
        log::info!("Vehicle PSClassName: {:?}", ps_name_result);
        if let Ok(name) = ps_name_result {
            log::info!("{}", name.as_str());
        }
    }
    ```
