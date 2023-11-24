#[doc = "Register `T0UPDATE` writer"]
pub type W = crate::W<T0UPDATE_SPEC>;
#[doc = "Field `UPDATE` writer - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
pub type UPDATE_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0UPDATE_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - Write any value will trigger a timer 0 time-base counter value update (timer 0 current value will be stored in registers above)"]
    #[inline(always)]
    #[must_use]
    pub fn update(&mut self) -> UPDATE_W<T0UPDATE_SPEC> {
        UPDATE_W::new(self, 0)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`t0update::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0UPDATE_SPEC;
impl crate::RegisterSpec for T0UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`t0update::W`](W) writer structure"]
impl crate::Writable for T0UPDATE_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets T0UPDATE to value 0"]
impl crate::Resettable for T0UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
