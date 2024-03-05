#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `READ_DONE_INT_CLR` writer - The clear signal for read_done interrupt."]
pub type READ_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PGM_DONE_INT_CLR` writer - The clear signal for pgm_done interrupt."]
pub type PGM_DONE_INT_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - The clear signal for read_done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn read_done_int_clr(&mut self) -> READ_DONE_INT_CLR_W<INT_CLR_SPEC> {
        READ_DONE_INT_CLR_W::new(self, 0)
    }
    #[doc = "Bit 1 - The clear signal for pgm_done interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn pgm_done_int_clr(&mut self) -> PGM_DONE_INT_CLR_W<INT_CLR_SPEC> {
        PGM_DONE_INT_CLR_W::new(self, 1)
    }
}
#[doc = "eFuse interrupt clear register.\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
