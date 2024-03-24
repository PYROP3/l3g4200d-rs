# l3g4200d-rs

Rust interface for the L3G4200D 3-axis gyroscope. 

Based on the chip datasheet I managed to find online, but it is still somewhat incomplete. Use at your own risk.

## Usage

### Struct initialization

```rs
use l3g4200d::gyro::{L3G4200D, L3G4200D_DEV_ID};
let mut gyro: L3G4200D = L3G4200D::new(1);
```

### Setup registers

```rs
gyro.common_setup().expect("Failed to setup gyro");
```

### Device validation

```rs
let who_am_i = gyro.who_am_i().expect("Failed to communicate with device");
assert_eq!(who_am_i, L3G4200D_DEV_ID, "Device does not seem to be L3G4200D");
```

### Drift compensation

My device would output a constant non-zero value even when stopped. I'm unsure if that's expected or caused by the rotation of the Earth itself, but in any case, `callibrate_drift` takes the average of N samples, and uses it to compensate the values returned by `read_position`

```rs
let callibration_data = gyroscope.callibrate_drift(200).expect("Failed to callibrate drift");
println!("Callibration finished: {:?}", callibration_data);
```

### Data collection

```rs
loop {
    let (x, y, z) = gyro.read_position().expect("Failed to read position data");
    println!("X: {}, Y: {}, Z: {}", x, y, z);
}
```

`read_position` and `read_position_filtered` are used to read the "current" angles of rotation (in degrees). Keep in mind that the sensor may have a slight drift, so use the filtered function to account for the observed drift.

`read_raw_delta` and `read_delta_filtered` are used to read the "difference" in rotation since the last call. This can be seen as the time derivative of the angles. Keep in mind that this value is raw from the sensor, and usually has to be scaled to be useful (to be interpreted as degrees or radians, for example).

## Known issues

This lib has been developed for use with a Raspberry Pi, through the I2C-1 channel. Therefore, read/write access to /dev/mem is necessary.
