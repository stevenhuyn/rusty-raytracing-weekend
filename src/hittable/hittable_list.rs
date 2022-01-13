use super::{HitRecord, Hittable};

struct HittableList {
    objects: Vec<dyn Hittable>,
}

impl HittableList {
    fn new() -> HittableList {
        HittableList {
            objects: Vec::new(),
        }
    }

    fn add(&self, object: dyn Hittable) {
        self.objects.push(object)
    }

    fn clear(&self) {
        self.objects.clear();
    }
}

impl Hittable for HittableList {
    fn hit(&self, ray: Ray, t_min: f64, t_max: f64, hit_record: &mut HitRecord) -> bool {
        let mut temp_record: HitRecord;
        let mut hit_anything = false;
        let mut closest_so_far = t_max;

        for object in self.objects {
            if object.hit(ray, t_min, t_max, &mut temp_record) {
                hit_anything = true;
                closest_so_far = temp_rec.t;
                hit_record = temp_rec;
            }
        }

        hit_anything
    }
}
