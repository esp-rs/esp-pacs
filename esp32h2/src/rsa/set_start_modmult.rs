#[doc = "Register `SET_START_MODMULT` writer"]
pub type W = crate::W<SET_START_MODMULT_SPEC>;
#[doc = "Field `SET_START_MODMULT` writer - Configure whether or not to start the modular multiplication. 0: No effect 1: Start"]
pub type SET_START_MODMULT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_START_MODMULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to start the modular multiplication. 0: No effect 1: Start"]
    #[inline(always)]
    #[must_use]
    pub fn set_start_modmult(&mut self) -> SET_START_MODMULT_W<SET_START_MODMULT_SPEC, 0> {
        SET_START_MODMULT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Starts modular multiplication\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_modmult::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_START_MODMULT_SPEC;
impl crate::RegisterSpec for SET_START_MODMULT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_modmult::W`](W) writer structure"]
impl crate::Writable for SET_START_MODMULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_START_MODMULT to value 0"]
impl crate::Resettable for SET_START_MODMULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
