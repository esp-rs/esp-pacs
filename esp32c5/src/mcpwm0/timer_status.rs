#[doc = "Register `TIMER%s_STATUS` reader"]
pub type R = crate::R<TIMER_STATUS_SPEC>;
#[doc = "Field `TIMER_VALUE` reader - Represents current PWM timer%s counter value."]
pub type TIMER_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_DIRECTION` reader - Represents current PWM timer%s counter direction.\\\\0: Increment\\\\1: Decrement"]
pub type TIMER_DIRECTION_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Represents current PWM timer%s counter value."]
    #[inline(always)]
    pub fn timer_value(&self) -> TIMER_VALUE_R {
        TIMER_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Represents current PWM timer%s counter direction.\\\\0: Increment\\\\1: Decrement"]
    #[inline(always)]
    pub fn timer_direction(&self) -> TIMER_DIRECTION_R {
        TIMER_DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_STATUS")
            .field("timer_value", &self.timer_value())
            .field("timer_direction", &self.timer_direction())
            .finish()
    }
}
#[doc = "PWM timer%s status register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_status::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_STATUS_SPEC;
impl crate::RegisterSpec for TIMER_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_status::R`](R) reader structure"]
impl crate::Readable for TIMER_STATUS_SPEC {}
#[doc = "`reset()` method sets TIMER%s_STATUS to value 0"]
impl crate::Resettable for TIMER_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
