#[doc = "Register `TIMER1_STATUS` reader"]
pub struct R(crate::R<TIMER1_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER1_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER1_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER1_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER1_VALUE` reader - "]
pub type TIMER1_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER1_DIRECTION` reader - "]
pub type TIMER1_DIRECTION_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer1_value(&self) -> TIMER1_VALUE_R {
        TIMER1_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn timer1_direction(&self) -> TIMER1_DIRECTION_R {
        TIMER1_DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER1_STATUS")
            .field(
                "timer1_value",
                &format_args!("{}", self.timer1_value().bits()),
            )
            .field(
                "timer1_direction",
                &format_args!("{}", self.timer1_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER1_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer1_status](index.html) module"]
pub struct TIMER1_STATUS_SPEC;
impl crate::RegisterSpec for TIMER1_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer1_status::R](R) reader structure"]
impl crate::Readable for TIMER1_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER1_STATUS to value 0"]
impl crate::Resettable for TIMER1_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
