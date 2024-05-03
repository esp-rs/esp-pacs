#[doc = "Register `STATUS` reader"]
pub type R = crate::R<STATUS_SPEC>;
#[doc = "Field `VALUE` reader - Represents current PWM timer%s counter value."]
pub type VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `DIRECTION` reader - Represents current PWM timer%s counter direction.\\\\0: Increment\\\\1: Decrement"]
pub type DIRECTION_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15 - Represents current PWM timer%s counter value."]
    #[inline(always)]
    pub fn value(&self) -> VALUE_R {
        VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16 - Represents current PWM timer%s counter direction.\\\\0: Increment\\\\1: Decrement"]
    #[inline(always)]
    pub fn direction(&self) -> DIRECTION_R {
        DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("value", &self.value().bits())
            .field("direction", &self.direction().bit())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "PWM timer0 status register.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`status::R`](R) reader structure"]
impl crate::Readable for STATUS_SPEC {}
#[doc = "`reset()` method sets STATUS to value 0"]
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
