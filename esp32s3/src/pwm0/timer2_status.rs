#[doc = "Register `TIMER2_STATUS` reader"]
pub struct R(crate::R<TIMER2_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<TIMER2_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<TIMER2_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<TIMER2_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `TIMER2_VALUE` reader - current PWM timer2 counter value"]
pub type TIMER2_VALUE_R = crate::FieldReader<u16, u16>;
#[doc = "Field `TIMER2_DIRECTION` reader - current PWM timer2 counter direction, 0: increment 1: decrement"]
pub type TIMER2_DIRECTION_R = crate::BitReader<bool>;
impl R {
    #[doc = "Bits 0:15 - current PWM timer2 counter value"]
    #[inline(always)]
    pub fn timer2_value(&self) -> TIMER2_VALUE_R {
        TIMER2_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - current PWM timer2 counter direction, 0: increment 1: decrement"]
    #[inline(always)]
    pub fn timer2_direction(&self) -> TIMER2_DIRECTION_R {
        TIMER2_DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[doc = "PWM timer2 status register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [timer2_status](index.html) module"]
pub struct TIMER2_STATUS_SPEC;
impl crate::RegisterSpec for TIMER2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [timer2_status::R](R) reader structure"]
impl crate::Readable for TIMER2_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets TIMER2_STATUS to value 0"]
impl crate::Resettable for TIMER2_STATUS_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
