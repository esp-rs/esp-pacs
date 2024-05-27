///Register `WDATAB` writer
pub type W = crate::W<WDATAB_SPEC>;
///Field `WDATAB` writer - NA
pub type WDATAB_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
///Field `WDATA_END` writer - NA
pub type WDATA_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDATAB_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:7 - NA
    #[inline(always)]
    #[must_use]
    pub fn wdatab(&mut self) -> WDATAB_W<WDATAB_SPEC> {
        WDATAB_W::new(self, 0)
    }
    ///Bit 8 - NA
    #[inline(always)]
    #[must_use]
    pub fn wdata_end(&mut self) -> WDATA_END_W<WDATAB_SPEC> {
        WDATA_END_W::new(self, 8)
    }
}
/**NA

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`wdatab::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WDATAB_SPEC;
impl crate::RegisterSpec for WDATAB_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`wdatab::W`](W) writer structure
impl crate::Writable for WDATAB_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets WDATAB to value 0
impl crate::Resettable for WDATAB_SPEC {
    const RESET_VALUE: u32 = 0;
}
