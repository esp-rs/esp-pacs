///Register `HPCORE_WDT_RESET_SOURCE0` reader
pub type R = crate::R<HPCORE_WDT_RESET_SOURCE0_SPEC>;
///Register `HPCORE_WDT_RESET_SOURCE0` writer
pub type W = crate::W<HPCORE_WDT_RESET_SOURCE0_SPEC>;
///Field `HPCORE0_WDT_RESET_SOURCE_SEL` reader - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0
pub type HPCORE0_WDT_RESET_SOURCE_SEL_R = crate::BitReader;
///Field `HPCORE0_WDT_RESET_SOURCE_SEL` writer - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0
pub type HPCORE0_WDT_RESET_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `HPCORE1_WDT_RESET_SOURCE_SEL` reader - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1
pub type HPCORE1_WDT_RESET_SOURCE_SEL_R = crate::BitReader;
///Field `HPCORE1_WDT_RESET_SOURCE_SEL` writer - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1
pub type HPCORE1_WDT_RESET_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0
    #[inline(always)]
    pub fn hpcore0_wdt_reset_source_sel(&self) -> HPCORE0_WDT_RESET_SOURCE_SEL_R {
        HPCORE0_WDT_RESET_SOURCE_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1
    #[inline(always)]
    pub fn hpcore1_wdt_reset_source_sel(&self) -> HPCORE1_WDT_RESET_SOURCE_SEL_R {
        HPCORE1_WDT_RESET_SOURCE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE_WDT_RESET_SOURCE0")
            .field(
                "hpcore0_wdt_reset_source_sel",
                &self.hpcore0_wdt_reset_source_sel(),
            )
            .field(
                "hpcore1_wdt_reset_source_sel",
                &self.hpcore1_wdt_reset_source_sel(),
            )
            .finish()
    }
}
impl W {
    ///Bit 0 - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0
    #[inline(always)]
    #[must_use]
    pub fn hpcore0_wdt_reset_source_sel(
        &mut self,
    ) -> HPCORE0_WDT_RESET_SOURCE_SEL_W<HPCORE_WDT_RESET_SOURCE0_SPEC> {
        HPCORE0_WDT_RESET_SOURCE_SEL_W::new(self, 0)
    }
    ///Bit 1 - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1
    #[inline(always)]
    #[must_use]
    pub fn hpcore1_wdt_reset_source_sel(
        &mut self,
    ) -> HPCORE1_WDT_RESET_SOURCE_SEL_W<HPCORE_WDT_RESET_SOURCE0_SPEC> {
        HPCORE1_WDT_RESET_SOURCE_SEL_W::new(self, 1)
    }
}
/**Reserved

You can [`read`](crate::generic::Reg::read) this register and get [`hpcore_wdt_reset_source0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`hpcore_wdt_reset_source0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HPCORE_WDT_RESET_SOURCE0_SPEC;
impl crate::RegisterSpec for HPCORE_WDT_RESET_SOURCE0_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hpcore_wdt_reset_source0::R`](R) reader structure
impl crate::Readable for HPCORE_WDT_RESET_SOURCE0_SPEC {}
///`write(|w| ..)` method takes [`hpcore_wdt_reset_source0::W`](W) writer structure
impl crate::Writable for HPCORE_WDT_RESET_SOURCE0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets HPCORE_WDT_RESET_SOURCE0 to value 0x02
impl crate::Resettable for HPCORE_WDT_RESET_SOURCE0_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
