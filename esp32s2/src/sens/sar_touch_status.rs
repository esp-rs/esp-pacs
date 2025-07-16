#[doc = "Register `SAR_TOUCH_STATUS%s` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS_SPEC>;
#[doc = "Field `DATA` reader - The data of touch pad 1, depending on the setting of SENS_TOUCH_DATA_SEL."]
pub type DATA_R = crate::FieldReader<u32>;
#[doc = "Field `DEBOUNCE` reader - Touch pad 1 debounce value."]
pub type DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - The data of touch pad 1, depending on the setting of SENS_TOUCH_DATA_SEL."]
    #[inline(always)]
    pub fn data(&self) -> DATA_R {
        DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - Touch pad 1 debounce value."]
    #[inline(always)]
    pub fn debounce(&self) -> DEBOUNCE_R {
        DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS")
            .field("data", &self.data())
            .field("debounce", &self.debounce())
            .finish()
    }
}
#[doc = "Touch channel status\n\nYou can [`read`](crate::Reg::read) this register and get [`sar_touch_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS%s to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS_SPEC {}
