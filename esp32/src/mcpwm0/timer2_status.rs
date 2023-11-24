#[doc = "Register `TIMER2_STATUS` reader"]
pub type R = crate::R<TIMER2_STATUS_SPEC>;
#[doc = "Field `TIMER2_VALUE` reader - "]
pub type TIMER2_VALUE_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER2_DIRECTION` reader - "]
pub type TIMER2_DIRECTION_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:15"]
    #[inline(always)]
    pub fn timer2_value(&self) -> TIMER2_VALUE_R {
        TIMER2_VALUE_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn timer2_direction(&self) -> TIMER2_DIRECTION_R {
        TIMER2_DIRECTION_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER2_STATUS")
            .field(
                "timer2_value",
                &format_args!("{}", self.timer2_value().bits()),
            )
            .field(
                "timer2_direction",
                &format_args!("{}", self.timer2_direction().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TIMER2_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`timer2_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER2_STATUS_SPEC;
impl crate::RegisterSpec for TIMER2_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer2_status::R`](R) reader structure"]
impl crate::Readable for TIMER2_STATUS_SPEC {}
#[doc = "`reset()` method sets TIMER2_STATUS to value 0"]
impl crate::Resettable for TIMER2_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
