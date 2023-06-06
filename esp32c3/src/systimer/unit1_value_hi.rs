#[doc = "Register `UNIT1_VALUE_HI` reader"]
pub struct R(crate::R<UNIT1_VALUE_HI_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<UNIT1_VALUE_HI_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<UNIT1_VALUE_HI_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<UNIT1_VALUE_HI_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER_UNIT1_VALUE_HI` reader - timer read value high 32bit"]
pub type TIMER_UNIT1_VALUE_HI_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - timer read value high 32bit"]
    #[inline(always)]
    pub fn timer_unit1_value_hi(&self) -> TIMER_UNIT1_VALUE_HI_R {
        TIMER_UNIT1_VALUE_HI_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UNIT1_VALUE_HI")
            .field(
                "timer_unit1_value_hi",
                &format_args!("{}", self.timer_unit1_value_hi().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<UNIT1_VALUE_HI_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "SYSTIMER_UNIT1_VALUE_HI.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [unit1_value_hi](index.html) module"]
pub struct UNIT1_VALUE_HI_SPEC;
impl crate::RegisterSpec for UNIT1_VALUE_HI_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [unit1_value_hi::R](R) reader structure"]
impl crate::Readable for UNIT1_VALUE_HI_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets UNIT1_VALUE_HI to value 0"]
impl crate::Resettable for UNIT1_VALUE_HI_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
