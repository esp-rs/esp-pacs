#[doc = "Register `MULT_INT_CLR` writer"]
pub type W = crate::W<MULT_INT_CLR_SPEC>;
#[doc = "Field `MULT_CALC_DONE` writer - Write 1 to clear the ECC_CALC_DONE_INT interrupt."]
pub type MULT_CALC_DONE_W<'a, REG> = crate::BitWriter1C<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MULT_INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to clear the ECC_CALC_DONE_INT interrupt."]
    #[inline(always)]
    pub fn mult_calc_done(&mut self) -> MULT_CALC_DONE_W<'_, MULT_INT_CLR_SPEC> {
        MULT_CALC_DONE_W::new(self, 0)
    }
}
#[doc = "ECC interrupt clear register\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mult_int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MULT_INT_CLR_SPEC;
impl crate::RegisterSpec for MULT_INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`mult_int_clr::W`](W) writer structure"]
impl crate::Writable for MULT_INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0x01;
}
#[doc = "`reset()` method sets MULT_INT_CLR to value 0"]
impl crate::Resettable for MULT_INT_CLR_SPEC {}
