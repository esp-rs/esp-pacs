#[doc = "Register `WR_JTAG` writer"]
pub type W = crate::W<WR_JTAG_SPEC>;
#[doc = "Field `WR_JTAG` writer - 32-bit of key to be compared."]
pub type WR_JTAG_W<'a, REG, const O: u8> = crate::FieldWriter<'a, REG, 32, O, u32>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WR_JTAG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:31 - 32-bit of key to be compared."]
    #[inline(always)]
    #[must_use]
    pub fn wr_jtag(&mut self) -> WR_JTAG_W<WR_JTAG_SPEC, 0> {
        WR_JTAG_W::new(self)
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
#[doc = "Jtag register 1.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wr_jtag::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WR_JTAG_SPEC;
impl crate::RegisterSpec for WR_JTAG_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wr_jtag::W`](W) writer structure"]
impl crate::Writable for WR_JTAG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WR_JTAG to value 0"]
impl crate::Resettable for WR_JTAG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
