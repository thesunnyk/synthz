
use lv2_raw::core;

pub struct LV2_Feature_Iter {
    next: *const *const core::LV2_Feature
}

impl LV2_Feature_Iter {
    pub fn new(features: *const *const core::LV2_Feature) -> LV2_Feature_Iter {
        LV2_Feature_Iter {
            next: features
        }
    }
}

impl Iterator for LV2_Feature_Iter {
    type Item = *const core::LV2_Feature;

    fn next(&mut self) -> Option<*const core::LV2_Feature> {
        unsafe {
            let retVal = *self.next;
            if retVal as usize == 0 {
                None
            } else {
                self.next = self.next.offset(1);
                Some(retVal)
            }
        }
    }

}

