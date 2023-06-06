#[doc = "Register `LOWPULSE` reader"]
pub struct R(crate::R<LOWPULSE_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<LOWPULSE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<LOWPULSE_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<LOWPULSE_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MIN_CNT` reader - This register stores the value of the minimum duration time for the low level pulse. it is used in baudrate-detect process."]
pub type MIN_CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - This register stores the value of the minimum duration time for the low level pulse. it is used in baudrate-detect process."]
    #[inline(always)]
    pub fn min_cnt(&self) -> MIN_CNT_R {
        MIN_CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LOWPULSE")
            .field("min_cnt", &format_args!("{}", self.min_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LOWPULSE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [lowpulse](index.html) module"]
pub struct LOWPULSE_SPEC;
impl crate::RegisterSpec for LOWPULSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [lowpulse::R](R) reader structure"]
impl crate::Readable for LOWPULSE_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets LOWPULSE to value 0x000f_ffff"]
impl crate::Resettable for LOWPULSE_SPEC {
    const RESET_VALUE: Self::Ux = 0x000f_ffff;
}
