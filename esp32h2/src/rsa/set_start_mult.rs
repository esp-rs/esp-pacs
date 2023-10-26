#[doc = "Register `SET_START_MULT` writer"]
pub type W = crate::W<SET_START_MULT_SPEC>;
#[doc = "Field `SET_START_MULT` writer - Configure whether or not to start the multiplication. 0: No effect 1: Start"]
pub type SET_START_MULT_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SET_START_MULT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Configure whether or not to start the multiplication. 0: No effect 1: Start"]
    #[inline(always)]
    #[must_use]
    pub fn set_start_mult(&mut self) -> SET_START_MULT_W<SET_START_MULT_SPEC, 0> {
        SET_START_MULT_W::new(self)
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
#[doc = "Starts multiplication\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`set_start_mult::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SET_START_MULT_SPEC;
impl crate::RegisterSpec for SET_START_MULT_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`set_start_mult::W`](W) writer structure"]
impl crate::Writable for SET_START_MULT_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SET_START_MULT to value 0"]
impl crate::Resettable for SET_START_MULT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
