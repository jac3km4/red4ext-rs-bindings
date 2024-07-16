use crate::{EulerAngles, Quaternion, Vector2, Vector3, Vector4};

impl From<mint::Vector2<f32>> for Vector2 {
    fn from(value: mint::Vector2<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<Vector2> for mint::Vector2<f32> {
    fn from(value: Vector2) -> Self {
        Self {
            x: value.x,
            y: value.y,
        }
    }
}

impl From<mint::Vector3<f32>> for Vector3 {
    fn from(value: mint::Vector3<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<Vector3> for mint::Vector3<f32> {
    fn from(value: Vector3) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
        }
    }
}

impl From<mint::Vector4<f32>> for Vector4 {
    fn from(value: mint::Vector4<f32>) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<Vector4> for mint::Vector4<f32> {
    fn from(value: Vector4) -> Self {
        Self {
            x: value.x,
            y: value.y,
            z: value.z,
            w: value.w,
        }
    }
}

impl From<mint::Quaternion<f32>> for Quaternion {
    fn from(value: mint::Quaternion<f32>) -> Self {
        Self {
            i: value.v.x,
            j: value.v.y,
            k: value.v.z,
            r: value.s,
        }
    }
}

impl From<Quaternion> for mint::Quaternion<f32> {
    fn from(value: Quaternion) -> Self {
        Self {
            v: mint::Vector3 {
                x: value.i,
                y: value.j,
                z: value.k,
            },
            s: value.r,
        }
    }
}

impl<B> From<mint::EulerAngles<f32, B>> for EulerAngles {
    fn from(value: mint::EulerAngles<f32, B>) -> Self {
        Self {
            roll: value.c,
            pitch: value.a,
            yaw: value.b,
        }
    }
}

impl<B> From<EulerAngles> for mint::EulerAngles<f32, B> {
    fn from(value: EulerAngles) -> Self {
        Self {
            c: value.roll,
            a: value.pitch,
            b: value.yaw,
            marker: std::marker::PhantomData,
        }
    }
}
