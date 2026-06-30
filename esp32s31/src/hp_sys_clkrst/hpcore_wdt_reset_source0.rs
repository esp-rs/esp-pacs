#[doc = "Register `HPCORE_WDT_RESET_SOURCE0` reader"]
pub type R = crate::R<HPCORE_WDT_RESET_SOURCE0_SPEC>;
#[doc = "Register `HPCORE_WDT_RESET_SOURCE0` writer"]
pub type W = crate::W<HPCORE_WDT_RESET_SOURCE0_SPEC>;
#[doc = "Field `REG_HPCORE0_WDT_RESET_SOURCE_SEL` reader - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
pub type REG_HPCORE0_WDT_RESET_SOURCE_SEL_R = crate::BitReader;
#[doc = "Field `REG_HPCORE0_WDT_RESET_SOURCE_SEL` writer - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
pub type REG_HPCORE0_WDT_RESET_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_HPCORE1_WDT_RESET_SOURCE_SEL` reader - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
pub type REG_HPCORE1_WDT_RESET_SOURCE_SEL_R = crate::BitReader;
#[doc = "Field `REG_HPCORE1_WDT_RESET_SOURCE_SEL` writer - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
pub type REG_HPCORE1_WDT_RESET_SOURCE_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
    #[inline(always)]
    pub fn reg_hpcore0_wdt_reset_source_sel(&self) -> REG_HPCORE0_WDT_RESET_SOURCE_SEL_R {
        REG_HPCORE0_WDT_RESET_SOURCE_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
    #[inline(always)]
    pub fn reg_hpcore1_wdt_reset_source_sel(&self) -> REG_HPCORE1_WDT_RESET_SOURCE_SEL_R {
        REG_HPCORE1_WDT_RESET_SOURCE_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE_WDT_RESET_SOURCE0")
            .field(
                "reg_hpcore0_wdt_reset_source_sel",
                &self.reg_hpcore0_wdt_reset_source_sel(),
            )
            .field(
                "reg_hpcore1_wdt_reset_source_sel",
                &self.reg_hpcore1_wdt_reset_source_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 1'b0: use wdt0 to reset hpcore0, 1'b1: use wdt1 to reset hpcore0"]
    #[inline(always)]
    pub fn reg_hpcore0_wdt_reset_source_sel(
        &mut self,
    ) -> REG_HPCORE0_WDT_RESET_SOURCE_SEL_W<'_, HPCORE_WDT_RESET_SOURCE0_SPEC> {
        REG_HPCORE0_WDT_RESET_SOURCE_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - 1'b0: use wdt0 to reset hpcore1, 1'b1: use wdt1 to reset hpcore1"]
    #[inline(always)]
    pub fn reg_hpcore1_wdt_reset_source_sel(
        &mut self,
    ) -> REG_HPCORE1_WDT_RESET_SOURCE_SEL_W<'_, HPCORE_WDT_RESET_SOURCE0_SPEC> {
        REG_HPCORE1_WDT_RESET_SOURCE_SEL_W::new(self, 1)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore_wdt_reset_source0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore_wdt_reset_source0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCORE_WDT_RESET_SOURCE0_SPEC;
impl crate::RegisterSpec for HPCORE_WDT_RESET_SOURCE0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore_wdt_reset_source0::R`](R) reader structure"]
impl crate::Readable for HPCORE_WDT_RESET_SOURCE0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcore_wdt_reset_source0::W`](W) writer structure"]
impl crate::Writable for HPCORE_WDT_RESET_SOURCE0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE_WDT_RESET_SOURCE0 to value 0x02"]
impl crate::Resettable for HPCORE_WDT_RESET_SOURCE0_SPEC {
    const RESET_VALUE: u32 = 0x02;
}
