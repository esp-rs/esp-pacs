#[doc = "Register `MAIN_BUF0_HIGH` reader"]
pub struct R(crate::R<MAIN_BUF0_HIGH_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MAIN_BUF0_HIGH_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MAIN_BUF0_HIGH_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MAIN_BUF0_HIGH_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `MAIN_TIMER_BUF0_HIGH` reader - need_des"]
pub type MAIN_TIMER_BUF0_HIGH_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - need_des"]
    #[inline(always)]
    pub fn main_timer_buf0_high(&self) -> MAIN_TIMER_BUF0_HIGH_R {
        MAIN_TIMER_BUF0_HIGH_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MAIN_BUF0_HIGH")
            .field(
                "main_timer_buf0_high",
                &format_args!("{}", self.main_timer_buf0_high().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MAIN_BUF0_HIGH_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [main_buf0_high](index.html) module"]
pub struct MAIN_BUF0_HIGH_SPEC;
impl crate::RegisterSpec for MAIN_BUF0_HIGH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [main_buf0_high::R](R) reader structure"]
impl crate::Readable for MAIN_BUF0_HIGH_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MAIN_BUF0_HIGH to value 0"]
impl crate::Resettable for MAIN_BUF0_HIGH_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
