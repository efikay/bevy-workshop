### 500ms and sprite is gone automatically along with the whole bundle(?):

```rust
commands.spawn((
    Timed::new(Duration::from_millis(500)),
    Sprite { .. },
)).observe(Timer::despawn_on_finished);

// you can then use Timed::frac() in queries, a scaled value from 0 to 1 based on its lifetime
```
