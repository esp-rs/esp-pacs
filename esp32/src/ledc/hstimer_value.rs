#[doc = "Register `HSTIMER%s_VALUE` reader"]
pub type R = crate::R<HSTIMER_VALUE_SPEC>;
#[doc = "Field `CNT` reader - software can read this register to get the current counter value in high speed timer0"]
pub type CNT_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19 - software can read this register to get the current counter value in high speed timer0"]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HSTIMER_VALUE")
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<HSTIMER_VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`hstimer_value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HSTIMER_VALUE_SPEC;
impl crate::RegisterSpec for HSTIMER_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hstimer_value::R`](R) reader structure"]
impl crate::Readable for HSTIMER_VALUE_SPEC {}
#[doc = "`reset()` method sets HSTIMER%s_VALUE to value 0"]
impl crate::Resettable for HSTIMER_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
