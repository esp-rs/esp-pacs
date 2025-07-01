#[doc = "Register `WDATAB` writer"]
pub type W = crate::W<WDATAB_SPEC>;
#[doc = "Field `WDATAB` writer - NA"]
pub type WDATAB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDATA_END` writer - NA"]
pub type WDATA_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDATAB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bits 0:7 - NA"]
    #[inline(always)]
    pub fn wdatab(&mut self) -> WDATAB_W<WDATAB_SPEC> {
        WDATAB_W::new(self, 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn wdata_end(&mut self) -> WDATA_END_W<WDATAB_SPEC> {
        WDATA_END_W::new(self, 8)
    }
}
#[doc = "NA\n\nYou can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`wdatab::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WDATAB_SPEC;
impl crate::RegisterSpec for WDATAB_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`wdatab::W`](W) writer structure"]
impl crate::Writable for WDATAB_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets WDATAB to value 0"]
impl crate::Resettable for WDATAB_SPEC {}
