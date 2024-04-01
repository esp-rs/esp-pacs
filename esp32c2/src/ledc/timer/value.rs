#[doc = "Register `VALUE` reader"]
pub type R = crate::R<VALUE_SPEC>;
#[doc = "Field `CNT` reader - This register stores the current counter value of timer %s."]
pub type CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:13 - This register stores the current counter value of timer %s."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0x3fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VALUE")
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<VALUE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Timer 0 current counter value\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`value::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VALUE_SPEC;
impl crate::RegisterSpec for VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`value::R`](R) reader structure"]
impl crate::Readable for VALUE_SPEC {}
#[doc = "`reset()` method sets VALUE to value 0"]
impl crate::Resettable for VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
