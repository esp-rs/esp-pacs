#[doc = "Register `SAR_TOUCH_STATUS3` reader"]
pub type R = crate::R<SAR_TOUCH_STATUS3_SPEC>;
#[doc = "Field `TOUCH_PAD3_DATA` reader - The data of touch pad 3, depending on the setting of SENS_TOUCH_DATA_SEL."]
pub type TOUCH_PAD3_DATA_R = crate::FieldReader<u32>;
#[doc = "Field `TOUCH_PAD3_DEBOUNCE` reader - Touch pad 3 debounce value."]
pub type TOUCH_PAD3_DEBOUNCE_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:21 - The data of touch pad 3, depending on the setting of SENS_TOUCH_DATA_SEL."]
    #[inline(always)]
    pub fn touch_pad3_data(&self) -> TOUCH_PAD3_DATA_R {
        TOUCH_PAD3_DATA_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bits 29:31 - Touch pad 3 debounce value."]
    #[inline(always)]
    pub fn touch_pad3_debounce(&self) -> TOUCH_PAD3_DEBOUNCE_R {
        TOUCH_PAD3_DEBOUNCE_R::new(((self.bits >> 29) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_TOUCH_STATUS3")
            .field(
                "touch_pad3_data",
                &format_args!("{}", self.touch_pad3_data().bits()),
            )
            .field(
                "touch_pad3_debounce",
                &format_args!("{}", self.touch_pad3_debounce().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_TOUCH_STATUS3_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Touch pad 3 status\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sar_touch_status3::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR_TOUCH_STATUS3_SPEC;
impl crate::RegisterSpec for SAR_TOUCH_STATUS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar_touch_status3::R`](R) reader structure"]
impl crate::Readable for SAR_TOUCH_STATUS3_SPEC {}
#[doc = "`reset()` method sets SAR_TOUCH_STATUS3 to value 0"]
impl crate::Resettable for SAR_TOUCH_STATUS3_SPEC {
    const RESET_VALUE: u32 = 0;
}
