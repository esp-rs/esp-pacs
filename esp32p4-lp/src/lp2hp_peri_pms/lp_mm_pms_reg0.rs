#[doc = "Register `LP_MM_PMS_REG0` reader"]
pub type R = crate::R<LP_MM_PMS_REG0_SPEC>;
#[doc = "Register `LP_MM_PMS_REG0` writer"]
pub type W = crate::W<LP_MM_PMS_REG0_SPEC>;
#[doc = "Field `REG_LP_MM_PSRAM_ALLOW` reader - NA"]
pub type REG_LP_MM_PSRAM_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_PSRAM_ALLOW` writer - NA"]
pub type REG_LP_MM_PSRAM_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_FLASH_ALLOW` reader - NA"]
pub type REG_LP_MM_FLASH_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_FLASH_ALLOW` writer - NA"]
pub type REG_LP_MM_FLASH_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_L2MEM_ALLOW` reader - NA"]
pub type REG_LP_MM_L2MEM_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_L2MEM_ALLOW` writer - NA"]
pub type REG_LP_MM_L2MEM_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_L2ROM_ALLOW` reader - NA"]
pub type REG_LP_MM_L2ROM_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_L2ROM_ALLOW` writer - NA"]
pub type REG_LP_MM_L2ROM_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_TRACE0_ALLOW` reader - NA"]
pub type REG_LP_MM_TRACE0_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_TRACE0_ALLOW` writer - NA"]
pub type REG_LP_MM_TRACE0_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_TRACE1_ALLOW` reader - NA"]
pub type REG_LP_MM_TRACE1_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_TRACE1_ALLOW` writer - NA"]
pub type REG_LP_MM_TRACE1_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_CPU_BUS_MON_ALLOW` reader - NA"]
pub type REG_LP_MM_CPU_BUS_MON_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_CPU_BUS_MON_ALLOW` writer - NA"]
pub type REG_LP_MM_CPU_BUS_MON_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_L2MEM_MON_ALLOW` reader - NA"]
pub type REG_LP_MM_L2MEM_MON_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_L2MEM_MON_ALLOW` writer - NA"]
pub type REG_LP_MM_L2MEM_MON_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_TCM_MON_ALLOW` reader - NA"]
pub type REG_LP_MM_TCM_MON_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_TCM_MON_ALLOW` writer - NA"]
pub type REG_LP_MM_TCM_MON_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_LP_MM_CACHE_ALLOW` reader - NA"]
pub type REG_LP_MM_CACHE_ALLOW_R = crate::BitReader;
#[doc = "Field `REG_LP_MM_CACHE_ALLOW` writer - NA"]
pub type REG_LP_MM_CACHE_ALLOW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_psram_allow(&self) -> REG_LP_MM_PSRAM_ALLOW_R {
        REG_LP_MM_PSRAM_ALLOW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_flash_allow(&self) -> REG_LP_MM_FLASH_ALLOW_R {
        REG_LP_MM_FLASH_ALLOW_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_l2mem_allow(&self) -> REG_LP_MM_L2MEM_ALLOW_R {
        REG_LP_MM_L2MEM_ALLOW_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_l2rom_allow(&self) -> REG_LP_MM_L2ROM_ALLOW_R {
        REG_LP_MM_L2ROM_ALLOW_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_trace0_allow(&self) -> REG_LP_MM_TRACE0_ALLOW_R {
        REG_LP_MM_TRACE0_ALLOW_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_trace1_allow(&self) -> REG_LP_MM_TRACE1_ALLOW_R {
        REG_LP_MM_TRACE1_ALLOW_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_cpu_bus_mon_allow(&self) -> REG_LP_MM_CPU_BUS_MON_ALLOW_R {
        REG_LP_MM_CPU_BUS_MON_ALLOW_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_l2mem_mon_allow(&self) -> REG_LP_MM_L2MEM_MON_ALLOW_R {
        REG_LP_MM_L2MEM_MON_ALLOW_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_tcm_mon_allow(&self) -> REG_LP_MM_TCM_MON_ALLOW_R {
        REG_LP_MM_TCM_MON_ALLOW_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_cache_allow(&self) -> REG_LP_MM_CACHE_ALLOW_R {
        REG_LP_MM_CACHE_ALLOW_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_MM_PMS_REG0")
            .field("reg_lp_mm_psram_allow", &self.reg_lp_mm_psram_allow())
            .field("reg_lp_mm_flash_allow", &self.reg_lp_mm_flash_allow())
            .field("reg_lp_mm_l2mem_allow", &self.reg_lp_mm_l2mem_allow())
            .field("reg_lp_mm_l2rom_allow", &self.reg_lp_mm_l2rom_allow())
            .field("reg_lp_mm_trace0_allow", &self.reg_lp_mm_trace0_allow())
            .field("reg_lp_mm_trace1_allow", &self.reg_lp_mm_trace1_allow())
            .field(
                "reg_lp_mm_cpu_bus_mon_allow",
                &self.reg_lp_mm_cpu_bus_mon_allow(),
            )
            .field(
                "reg_lp_mm_l2mem_mon_allow",
                &self.reg_lp_mm_l2mem_mon_allow(),
            )
            .field("reg_lp_mm_tcm_mon_allow", &self.reg_lp_mm_tcm_mon_allow())
            .field("reg_lp_mm_cache_allow", &self.reg_lp_mm_cache_allow())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_psram_allow(&mut self) -> REG_LP_MM_PSRAM_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_PSRAM_ALLOW_W::new(self, 0)
    }
    #[doc = "Bit 1 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_flash_allow(&mut self) -> REG_LP_MM_FLASH_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_FLASH_ALLOW_W::new(self, 1)
    }
    #[doc = "Bit 2 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_l2mem_allow(&mut self) -> REG_LP_MM_L2MEM_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_L2MEM_ALLOW_W::new(self, 2)
    }
    #[doc = "Bit 3 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_l2rom_allow(&mut self) -> REG_LP_MM_L2ROM_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_L2ROM_ALLOW_W::new(self, 3)
    }
    #[doc = "Bit 6 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_trace0_allow(&mut self) -> REG_LP_MM_TRACE0_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_TRACE0_ALLOW_W::new(self, 6)
    }
    #[doc = "Bit 7 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_trace1_allow(&mut self) -> REG_LP_MM_TRACE1_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_TRACE1_ALLOW_W::new(self, 7)
    }
    #[doc = "Bit 8 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_cpu_bus_mon_allow(
        &mut self,
    ) -> REG_LP_MM_CPU_BUS_MON_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_CPU_BUS_MON_ALLOW_W::new(self, 8)
    }
    #[doc = "Bit 9 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_l2mem_mon_allow(
        &mut self,
    ) -> REG_LP_MM_L2MEM_MON_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_L2MEM_MON_ALLOW_W::new(self, 9)
    }
    #[doc = "Bit 10 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_tcm_mon_allow(
        &mut self,
    ) -> REG_LP_MM_TCM_MON_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_TCM_MON_ALLOW_W::new(self, 10)
    }
    #[doc = "Bit 11 - NA"]
    #[inline(always)]
    pub fn reg_lp_mm_cache_allow(&mut self) -> REG_LP_MM_CACHE_ALLOW_W<'_, LP_MM_PMS_REG0_SPEC> {
        REG_LP_MM_CACHE_ALLOW_W::new(self, 11)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_mm_pms_reg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_mm_pms_reg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_MM_PMS_REG0_SPEC;
impl crate::RegisterSpec for LP_MM_PMS_REG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_mm_pms_reg0::R`](R) reader structure"]
impl crate::Readable for LP_MM_PMS_REG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_mm_pms_reg0::W`](W) writer structure"]
impl crate::Writable for LP_MM_PMS_REG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_MM_PMS_REG0 to value 0"]
impl crate::Resettable for LP_MM_PMS_REG0_SPEC {}
