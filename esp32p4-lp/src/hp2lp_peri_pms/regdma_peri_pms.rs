#[doc = "Register `REGDMA_PERI_PMS` reader"]
pub type R = crate::R<REGDMA_PERI_PMS_SPEC>;
#[doc = "Register `REGDMA_PERI_PMS` writer"]
pub type W = crate::W<REGDMA_PERI_PMS_SPEC>;
#[doc = "Field `REG_REGDMA_PERI_LP_RAM_ALLOW` reader - NA"]
pub type REG_REGDMA_PERI_LP_RAM_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_REGDMA_PERI_LP_RAM_ALLOW` writer - NA"]
pub type REG_REGDMA_PERI_LP_RAM_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_REGDMA_PERI_LP_PERI_ALLOW` reader - NA"]
pub type REG_REGDMA_PERI_LP_PERI_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_REGDMA_PERI_LP_PERI_ALLOW` writer - NA"]
pub type REG_REGDMA_PERI_LP_PERI_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_regdma_peri_lp_ram_allow(&self) -> REG_REGDMA_PERI_LP_RAM_ALLOW_R {
        REG_REGDMA_PERI_LP_RAM_ALLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_regdma_peri_lp_peri_allow(&self) -> REG_REGDMA_PERI_LP_PERI_ALLOW_R {
        REG_REGDMA_PERI_LP_PERI_ALLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGDMA_PERI_PMS")
            .field(
                "reg_regdma_peri_lp_ram_allow",
                &self.reg_regdma_peri_lp_ram_allow(),
            )
            .field(
                "reg_regdma_peri_lp_peri_allow",
                &self.reg_regdma_peri_lp_peri_allow(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_regdma_peri_lp_ram_allow(
        &mut self,
    ) -> REG_REGDMA_PERI_LP_RAM_ALLOW_W<'_, REGDMA_PERI_PMS_SPEC> {
        REG_REGDMA_PERI_LP_RAM_ALLOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_regdma_peri_lp_peri_allow(
        &mut self,
    ) -> REG_REGDMA_PERI_LP_PERI_ALLOW_W<'_, REGDMA_PERI_PMS_SPEC> {
        REG_REGDMA_PERI_LP_PERI_ALLOW_W::new(self, 1)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`regdma_peri_pms::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`regdma_peri_pms::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGDMA_PERI_PMS_SPEC;
impl crate::RegisterSpec for REGDMA_PERI_PMS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`regdma_peri_pms::R`](R) reader structure"]
impl crate::Readable for REGDMA_PERI_PMS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`regdma_peri_pms::W`](W) writer structure"]
impl crate::Writable for REGDMA_PERI_PMS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets REGDMA_PERI_PMS to value 0"]
impl crate::Resettable for REGDMA_PERI_PMS_SPEC {}
