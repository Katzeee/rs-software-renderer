pub mod buffer_attachment;
pub mod line;
pub mod math;
pub mod renderer;
pub mod shader;
pub mod triangle;

#[cfg(test)]
mod tests {

    use crate::{
        buffer_attachment::BufferAttachment,
        math::{Vec2, Vec3, Vec4},
        shader::lerp,
    };

    #[test]
    fn buffer_attachment_test() {
        let mut attachment1 = BufferAttachment::new(10, 10, f32::MAX);
        attachment1.set(0, 0, 10.0);
        assert_eq!(attachment1.get(0, 0), &10.0);
        let mut attachment1: BufferAttachment<Vec4<f32>> =
            BufferAttachment::new(2, 2, Vec4::new(0.0, 0.0, 0.0, 0.0));
        attachment1.get(0, 0).x = 9.0;
        assert_eq!(attachment1.get(0, 0).x, 9.0);
        assert_eq!(attachment1.get(0, 0).len_square(), 81.0);
        assert_eq!(attachment1.get(0, 0).len(), 9.0);
        // dbg!(attachment1.get(0, 0).normalize());
    }

    #[test]
    fn vec_test() {
        let mut v1: Vec4<f32> = Vec4::new(1.0, 2.0, 4.0, 8.0);
        let v2: Vec4<f32> = Vec4::new(1.0, 1.0, 1.0, 1.0);
        assert_eq!(v2.normalize(), Vec4::new(0.5, 0.5, 0.5, 0.5));
        assert_eq!(v2.len(), 2.0);
        assert_eq!(v1 + v2, Vec4::new(2.0, 3.0, 5.0, 9.0));
        assert_eq!(v1.len_square(), 85.0);
        assert_eq!(v1.len(), 9.219544457292887);
        v1.x = 8.0;
        assert_eq!(v1, Vec4::new(8.0, 2.0, 4.0, 8.0));
        let _v3: Vec3<i32> = Vec3::new(1, 2, 3);
        let _v4: Vec2<u32> = Vec2::new(1, 3);
    }

    #[test]
    fn lerp_test() {
        let s = 0.;
        let e = 1.;
        assert_eq!(lerp(s, e, 0.5), 0.5);
        let s = Vec2::from(0.);
        let e = Vec2::from(1.);
        assert_eq!(lerp(s, e, 0.5), Vec2::from(0.5));
    }
}
