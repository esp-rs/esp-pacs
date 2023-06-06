#[doc = "Register `TIMER0_STATUS` reader"]
pub struct R(crate::R<TIMER0_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER0_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER0_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER0_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER0_VALUE` reader - "]
pub type TIMER0_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER0_DIRECTION` reader - "]
pub type TIMER0_DIRECTION_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer0_value(&self) -> TIMER0_VALUE_R {
        TIMER0_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn timer0_direction(&self) -> TIMER0_DIRECTION_R {
        TIMER0_DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER0_STATUS")
            .field(
                "timer0_value",
                &format_args!("{}", self.timer0_value().bits()),
            )
            .field(
                "timer0_direction",
                &format_args!("{}", self.timer0_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER0_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer0_status](index.html) module"]
pub struct TIMER0_STATUS_SPEC;
impl crate::RegisterSpec for TIMER0_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer0_status::R](R) reader structure"]
impl crate::Readable for TIMER0_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER0_STATUS to value 0"]
impl crate::Resettable for TIMER0_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
