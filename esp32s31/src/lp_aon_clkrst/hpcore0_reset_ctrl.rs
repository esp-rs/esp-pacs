#[doc = "Register `HPCORE0_RESET_CTRL` reader"]
pub type R = crate::R<HPCORE0_RESET_CTRL_SPEC>;
#[doc = "Register `HPCORE0_RESET_CTRL` writer"]
pub type W = crate::W<HPCORE0_RESET_CTRL_SPEC>;
#[doc = "Field `HPCORE0_SW_STALL_CODE` reader - reserved"]
pub type HPCORE0_SW_STALL_CODE_R = crate::FieldReader;
#[doc = "Field `HPCORE0_SW_STALL_CODE` writer - reserved"]
pub type HPCORE0_SW_STALL_CODE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `HPCORE0_STALL_WAIT` reader - need_des"]
pub type HPCORE0_STALL_WAIT_R = crate::FieldReader;
#[doc = "Field `HPCORE0_STALL_WAIT` writer - need_des"]
pub type HPCORE0_STALL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `HPCORE0_STALL_EN` reader - need_des"]
pub type HPCORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `HPCORE0_STALL_EN` writer - need_des"]
pub type HPCORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_RESET_LENGTH` reader - need_des"]
pub type HPCORE0_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `HPCORE0_RESET_LENGTH` writer - need_des"]
pub type HPCORE0_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `HPCORE0_ENABLE_LP_WDT_RESET` reader - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
pub type HPCORE0_ENABLE_LP_WDT_RESET_R = crate::BitReader;
#[doc = "Field `HPCORE0_ENABLE_LP_WDT_RESET` writer - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
pub type HPCORE0_ENABLE_LP_WDT_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_SW_RESET` writer - need_des"]
pub type HPCORE0_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_OCD_HALT_ON_RESET` reader - need_des"]
pub type HPCORE0_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `HPCORE0_OCD_HALT_ON_RESET` writer - need_des"]
pub type HPCORE0_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_STAT_VECTOR_SEL` reader - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
pub type HPCORE0_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `HPCORE0_STAT_VECTOR_SEL` writer - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
pub type HPCORE0_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_LOCKUP_RESET_EN` reader - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
pub type HPCORE0_LOCKUP_RESET_EN_R = crate::BitReader;
#[doc = "Field `HPCORE0_LOCKUP_RESET_EN` writer - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
pub type HPCORE0_LOCKUP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn hpcore0_sw_stall_code(&self) -> HPCORE0_SW_STALL_CODE_R {
        HPCORE0_SW_STALL_CODE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:14 - need_des"]
    #[inline(always)]
    pub fn hpcore0_stall_wait(&self) -> HPCORE0_STALL_WAIT_R {
        HPCORE0_STALL_WAIT_R::new(((self.bits >> 8) & 0x7f) as u8)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn hpcore0_stall_en(&self) -> HPCORE0_STALL_EN_R {
        HPCORE0_STALL_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn hpcore0_reset_length(&self) -> HPCORE0_RESET_LENGTH_R {
        HPCORE0_RESET_LENGTH_R::new(((self.bits >> 16) & 7) as u8)
    }
    #[doc = "Bit 19 - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
    #[inline(always)]
    pub fn hpcore0_enable_lp_wdt_reset(&self) -> HPCORE0_ENABLE_LP_WDT_RESET_R {
        HPCORE0_ENABLE_LP_WDT_RESET_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hpcore0_ocd_halt_on_reset(&self) -> HPCORE0_OCD_HALT_ON_RESET_R {
        HPCORE0_OCD_HALT_ON_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn hpcore0_stat_vector_sel(&self) -> HPCORE0_STAT_VECTOR_SEL_R {
        HPCORE0_STAT_VECTOR_SEL_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
    #[inline(always)]
    pub fn hpcore0_lockup_reset_en(&self) -> HPCORE0_LOCKUP_RESET_EN_R {
        HPCORE0_LOCKUP_RESET_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE0_RESET_CTRL")
            .field("hpcore0_sw_stall_code", &self.hpcore0_sw_stall_code())
            .field("hpcore0_stall_wait", &self.hpcore0_stall_wait())
            .field("hpcore0_stall_en", &self.hpcore0_stall_en())
            .field("hpcore0_reset_length", &self.hpcore0_reset_length())
            .field(
                "hpcore0_enable_lp_wdt_reset",
                &self.hpcore0_enable_lp_wdt_reset(),
            )
            .field(
                "hpcore0_ocd_halt_on_reset",
                &self.hpcore0_ocd_halt_on_reset(),
            )
            .field("hpcore0_stat_vector_sel", &self.hpcore0_stat_vector_sel())
            .field("hpcore0_lockup_reset_en", &self.hpcore0_lockup_reset_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - reserved"]
    #[inline(always)]
    pub fn hpcore0_sw_stall_code(
        &mut self,
    ) -> HPCORE0_SW_STALL_CODE_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_SW_STALL_CODE_W::new(self, 0)
    }
    #[doc = "Bits 8:14 - need_des"]
    #[inline(always)]
    pub fn hpcore0_stall_wait(&mut self) -> HPCORE0_STALL_WAIT_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_STALL_WAIT_W::new(self, 8)
    }
    #[doc = "Bit 15 - need_des"]
    #[inline(always)]
    pub fn hpcore0_stall_en(&mut self) -> HPCORE0_STALL_EN_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_STALL_EN_W::new(self, 15)
    }
    #[doc = "Bits 16:18 - need_des"]
    #[inline(always)]
    pub fn hpcore0_reset_length(&mut self) -> HPCORE0_RESET_LENGTH_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_RESET_LENGTH_W::new(self, 16)
    }
    #[doc = "Bit 19 - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
    #[inline(always)]
    pub fn hpcore0_enable_lp_wdt_reset(
        &mut self,
    ) -> HPCORE0_ENABLE_LP_WDT_RESET_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_ENABLE_LP_WDT_RESET_W::new(self, 19)
    }
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn hpcore0_sw_reset(&mut self) -> HPCORE0_SW_RESET_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_SW_RESET_W::new(self, 20)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn hpcore0_ocd_halt_on_reset(
        &mut self,
    ) -> HPCORE0_OCD_HALT_ON_RESET_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_OCD_HALT_ON_RESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn hpcore0_stat_vector_sel(
        &mut self,
    ) -> HPCORE0_STAT_VECTOR_SEL_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_STAT_VECTOR_SEL_W::new(self, 30)
    }
    #[doc = "Bit 31 - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
    #[inline(always)]
    pub fn hpcore0_lockup_reset_en(
        &mut self,
    ) -> HPCORE0_LOCKUP_RESET_EN_W<'_, HPCORE0_RESET_CTRL_SPEC> {
        HPCORE0_LOCKUP_RESET_EN_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore0_reset_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore0_reset_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCORE0_RESET_CTRL_SPEC;
impl crate::RegisterSpec for HPCORE0_RESET_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore0_reset_ctrl::R`](R) reader structure"]
impl crate::Readable for HPCORE0_RESET_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcore0_reset_ctrl::W`](W) writer structure"]
impl crate::Writable for HPCORE0_RESET_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE0_RESET_CTRL to value 0x4001_0000"]
impl crate::Resettable for HPCORE0_RESET_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4001_0000;
}
