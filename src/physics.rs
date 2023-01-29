/// delta v = v2 - v1
pub fn displacement(v1: na::Vector3<f32>, v2: na::Vector3<f32>)
    -> na::Vector3<f32> {
   v2 - v1 
}

#[cfg(test)]
mod tests {
    use crate::physics;

    #[test]
    fn test_displacement() {
        let v1 =  na::Vector3::new(1., 0., 0.);
        let v2 =  na::Vector3::new(1., 2., 3.);
        assert_eq!(na::Vector3::new(0., 2., 3.), physics::displacement(v1, v2));
    }
}
