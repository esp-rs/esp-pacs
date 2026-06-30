#[doc = "Register `LP_AONCLKRST_HPCORE0_RESET_CTRL` reader"]
pub type R = crate::R<LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC>;
#[doc = "Register `LP_AONCLKRST_HPCORE0_RESET_CTRL` writer"]
pub type W = crate::W<LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_STALL_CODE` reader - reserved"]
pub type LP_AONCLKRST_HPCORE0_SW_STALL_CODE_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_STALL_CODE` writer - reserved"]
pub type LP_AONCLKRST_HPCORE0_SW_STALL_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_WAIT` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_WAIT_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_WAIT` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_EN` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_EN` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_RESET_LENGTH` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_RESET_LENGTH` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET` reader - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
pub type LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET` writer - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
pub type LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_SW_RESET` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL` reader - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
pub type LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL` writer - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
pub type LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN` reader - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
pub type LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN` writer - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
pub type LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(&self) -> LP_AONCLKRST_HPCORE0_SW_STALL_CODE_R {
        LP_AONCLKRST_HPCORE0_SW_STALL_CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_wait(&self) -> LP_AONCLKRST_HPCORE0_STALL_WAIT_R {
        LP_AONCLKRST_HPCORE0_STALL_WAIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_en(&self) -> LP_AONCLKRST_HPCORE0_STALL_EN_R {
        LP_AONCLKRST_HPCORE0_STALL_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_length(&self) -> LP_AONCLKRST_HPCORE0_RESET_LENGTH_R {
        LP_AONCLKRST_HPCORE0_RESET_LENGTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_enable_lp_wdt_reset(
        &self,
    ) -> LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET_R {
        LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_ocd_halt_on_reset(
        &self,
    ) -> LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_R {
        LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stat_vector_sel(&self) -> LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_R {
        LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_lockup_reset_en(&self) -> LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_R {
        LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HPCORE0_RESET_CTRL")
            .field(
                "lp_aonclkrst_hpcore0_sw_stall_code",
                &self.lp_aonclkrst_hpcore0_sw_stall_code(),
            )
            .field(
                "lp_aonclkrst_hpcore0_stall_wait",
                &self.lp_aonclkrst_hpcore0_stall_wait(),
            )
            .field(
                "lp_aonclkrst_hpcore0_stall_en",
                &self.lp_aonclkrst_hpcore0_stall_en(),
            )
            .field(
                "lp_aonclkrst_hpcore0_reset_length",
                &self.lp_aonclkrst_hpcore0_reset_length(),
            )
            .field(
                "lp_aonclkrst_hpcore0_enable_lp_wdt_reset",
                &self.lp_aonclkrst_hpcore0_enable_lp_wdt_reset(),
            )
            .field(
                "lp_aonclkrst_hpcore0_ocd_halt_on_reset",
                &self.lp_aonclkrst_hpcore0_ocd_halt_on_reset(),
            )
            .field(
                "lp_aonclkrst_hpcore0_stat_vector_sel",
                &self.lp_aonclkrst_hpcore0_stat_vector_sel(),
            )
            .field(
                "lp_aonclkrst_hpcore0_lockup_reset_en",
                &self.lp_aonclkrst_hpcore0_lockup_reset_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_stall_code(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_SW_STALL_CODE_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_SW_STALL_CODE_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_wait(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_STALL_WAIT_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_STALL_WAIT_W::new(self, 8)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_en(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_STALL_EN_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_STALL_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_reset_length(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_RESET_LENGTH_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_RESET_LENGTH_W::new(self, 16)
    }
    #[doc = "Bit 19 - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_enable_lp_wdt_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_ENABLE_LP_WDT_RESET_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_SW_RESET_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_SW_RESET_W::new(self, 20)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_ocd_halt_on_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stat_vector_sel(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_lockup_reset_en(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_W<'_, LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC> {
        LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcore0_reset_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcore0_reset_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hpcore0_reset_ctrl::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hpcore0_reset_ctrl::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HPCORE0_RESET_CTRL to value 0x4001_0000"]
impl crate::Resettable for LP_AONCLKRST_HPCORE0_RESET_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4001_0000;
}
