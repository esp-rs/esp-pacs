#[doc = "Register `SAR_TOUCH_STATUS7` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS7_SPEC>;
#[doc = "Field `TOUCH_PAD7_DATA` reader - The data of touch pad 7, depending on the setting of SENS_TOUCH_DATA_SEL."]
pub type TOUCH_PAD7_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_PAD7_DEBOUNCE` reader - Touch pad 7 debounce value."]
pub type TOUCH_PAD7_DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - The data of touch pad 7, depending on the setting of SENS_TOUCH_DATA_SEL."]
    #[inline(always)]
    pub fn touch_pad7_data(&self) -> TOUCH_PAD7_DATA_R {
        TOUCH_PAD7_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - Touch pad 7 debounce value."]
    #[inline(always)]
    pub fn touch_pad7_debounce(&self) -> TOUCH_PAD7_DEBOUNCE_R {
        TOUCH_PAD7_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS7")
            .field("touch_pad7_data", &self.touch_pad7_data().bits())
            .field("touch_pad7_debounce", &self.touch_pad7_debounce().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_STATUS7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Touch pad 7 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS7_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status7::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS7_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS7 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS7_SPEC {
    const RESET_VALUE: u32 = 0;
}
