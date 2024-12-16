#[doc = "Register `LP_AONCLKRST_HPCPU_RESET_CTRL0` reader"]
pub type R = crate::R<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC>;
#[doc = "Register `LP_AONCLKRST_HPCPU_RESET_CTRL0` writer"]
pub type W = crate::W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN` reader - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
pub type LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN` writer - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
pub type LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH` reader - need_des"]
pub type LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH` writer - need_des"]
pub type LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN` reader - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
pub type LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN` writer - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
pub type LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_WAIT` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_WAIT_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_WAIT` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_EN` reader - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE0_STALL_EN` writer - need_des"]
pub type LP_AONCLKRST_HPCORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
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
#[doc = "Field `LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN` reader - write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature"]
pub type LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN` writer - write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature"]
pub type LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH` reader - need_des"]
pub type LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH` writer - need_des"]
pub type LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN` reader - write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature"]
pub type LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN` writer - write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature"]
pub type LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_STALL_WAIT` reader - need_des"]
pub type LP_AONCLKRST_HPCORE1_STALL_WAIT_R = crate::FieldReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_STALL_WAIT` writer - need_des"]
pub type LP_AONCLKRST_HPCORE1_STALL_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 7>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_STALL_EN` reader - need_des"]
pub type LP_AONCLKRST_HPCORE1_STALL_EN_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_STALL_EN` writer - need_des"]
pub type LP_AONCLKRST_HPCORE1_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_SW_RESET` writer - need_des"]
pub type LP_AONCLKRST_HPCORE1_SW_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET` reader - need_des"]
pub type LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET` writer - need_des"]
pub type LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL` reader - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
pub type LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_R = crate::BitReader;
#[doc = "Field `LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL` writer - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
pub type LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_lockup_reset_en(&self) -> LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_R {
        LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:3 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore0_reset_length(
        &self,
    ) -> LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_R {
        LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_R::new(((self.bits >> 1) & 7) as u8)
    }
    #[doc = "Bit 4 - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore0_reset_en(&self) -> LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_R {
        LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:11 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_wait(&self) -> LP_AONCLKRST_HPCORE0_STALL_WAIT_R {
        LP_AONCLKRST_HPCORE0_STALL_WAIT_R::new(((self.bits >> 5) & 0x7f) as u8)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_en(&self) -> LP_AONCLKRST_HPCORE0_STALL_EN_R {
        LP_AONCLKRST_HPCORE0_STALL_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_ocd_halt_on_reset(
        &self,
    ) -> LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_R {
        LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stat_vector_sel(&self) -> LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_R {
        LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_lockup_reset_en(&self) -> LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_R {
        LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:19 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore1_reset_length(
        &self,
    ) -> LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_R {
        LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_R::new(((self.bits >> 17) & 7) as u8)
    }
    #[doc = "Bit 20 - write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore1_reset_en(&self) -> LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_R {
        LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bits 21:27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stall_wait(&self) -> LP_AONCLKRST_HPCORE1_STALL_WAIT_R {
        LP_AONCLKRST_HPCORE1_STALL_WAIT_R::new(((self.bits >> 21) & 0x7f) as u8)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stall_en(&self) -> LP_AONCLKRST_HPCORE1_STALL_EN_R {
        LP_AONCLKRST_HPCORE1_STALL_EN_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_ocd_halt_on_reset(
        &self,
    ) -> LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_R {
        LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stat_vector_sel(&self) -> LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_R {
        LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_AONCLKRST_HPCPU_RESET_CTRL0")
            .field(
                "lp_aonclkrst_hpcore0_lockup_reset_en",
                &self.lp_aonclkrst_hpcore0_lockup_reset_en(),
            )
            .field(
                "lp_aonclkrst_lp_wdt_hpcore0_reset_length",
                &self.lp_aonclkrst_lp_wdt_hpcore0_reset_length(),
            )
            .field(
                "lp_aonclkrst_lp_wdt_hpcore0_reset_en",
                &self.lp_aonclkrst_lp_wdt_hpcore0_reset_en(),
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
                "lp_aonclkrst_hpcore0_ocd_halt_on_reset",
                &self.lp_aonclkrst_hpcore0_ocd_halt_on_reset(),
            )
            .field(
                "lp_aonclkrst_hpcore0_stat_vector_sel",
                &self.lp_aonclkrst_hpcore0_stat_vector_sel(),
            )
            .field(
                "lp_aonclkrst_hpcore1_lockup_reset_en",
                &self.lp_aonclkrst_hpcore1_lockup_reset_en(),
            )
            .field(
                "lp_aonclkrst_lp_wdt_hpcore1_reset_length",
                &self.lp_aonclkrst_lp_wdt_hpcore1_reset_length(),
            )
            .field(
                "lp_aonclkrst_lp_wdt_hpcore1_reset_en",
                &self.lp_aonclkrst_lp_wdt_hpcore1_reset_en(),
            )
            .field(
                "lp_aonclkrst_hpcore1_stall_wait",
                &self.lp_aonclkrst_hpcore1_stall_wait(),
            )
            .field(
                "lp_aonclkrst_hpcore1_stall_en",
                &self.lp_aonclkrst_hpcore1_stall_en(),
            )
            .field(
                "lp_aonclkrst_hpcore1_ocd_halt_on_reset",
                &self.lp_aonclkrst_hpcore1_ocd_halt_on_reset(),
            )
            .field(
                "lp_aonclkrst_hpcore1_stat_vector_sel",
                &self.lp_aonclkrst_hpcore1_stat_vector_sel(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - write 1 to enable hpcore0 lockup reset feature, write 0 to disable hpcore0 lockup reset feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_lockup_reset_en(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE0_LOCKUP_RESET_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:3 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore0_reset_length(
        &mut self,
    ) -> LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_LP_WDT_HPCORE0_RESET_LENGTH_W::new(self, 1)
    }
    #[doc = "Bit 4 - write 1 to enable lp_wdt reset hpcore0 feature, write 0 to disable lp_wdt reset hpcore0 feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore0_reset_en(
        &mut self,
    ) -> LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_LP_WDT_HPCORE0_RESET_EN_W::new(self, 4)
    }
    #[doc = "Bits 5:11 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_wait(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_STALL_WAIT_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE0_STALL_WAIT_W::new(self, 5)
    }
    #[doc = "Bit 12 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stall_en(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_STALL_EN_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE0_STALL_EN_W::new(self, 12)
    }
    #[doc = "Bit 13 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_sw_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_SW_RESET_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE0_SW_RESET_W::new(self, 13)
    }
    #[doc = "Bit 14 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_ocd_halt_on_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE0_OCD_HALT_ON_RESET_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore0_stat_vector_sel(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE0_STAT_VECTOR_SEL_W::new(self, 15)
    }
    #[doc = "Bit 16 - write 1 to enable hpcore1 lockup reset feature, write 0 to disable hpcore1 lockup reset feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_lockup_reset_en(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE1_LOCKUP_RESET_EN_W::new(self, 16)
    }
    #[doc = "Bits 17:19 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore1_reset_length(
        &mut self,
    ) -> LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_LP_WDT_HPCORE1_RESET_LENGTH_W::new(self, 17)
    }
    #[doc = "Bit 20 - write 1 to enable lp_wdt reset hpcore1 feature, write 0 to disable lp_wdt reset hpcore1 feature"]
    #[inline(always)]
    pub fn lp_aonclkrst_lp_wdt_hpcore1_reset_en(
        &mut self,
    ) -> LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_LP_WDT_HPCORE1_RESET_EN_W::new(self, 20)
    }
    #[doc = "Bits 21:27 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stall_wait(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_STALL_WAIT_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE1_STALL_WAIT_W::new(self, 21)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stall_en(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_STALL_EN_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE1_STALL_EN_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_sw_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_SW_RESET_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE1_SW_RESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_ocd_halt_on_reset(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE1_OCD_HALT_ON_RESET_W::new(self, 30)
    }
    #[doc = "Bit 31 - 1'b1: boot from HP TCM ROM: 0x4FC00000 1'b0: boot from LP TCM RAM: 0x50108000"]
    #[inline(always)]
    pub fn lp_aonclkrst_hpcore1_stat_vector_sel(
        &mut self,
    ) -> LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_W<LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC> {
        LP_AONCLKRST_HPCORE1_STAT_VECTOR_SEL_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_aonclkrst_hpcpu_reset_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_aonclkrst_hpcpu_reset_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC;
impl crate::RegisterSpec for LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_aonclkrst_hpcpu_reset_ctrl0::R`](R) reader structure"]
impl crate::Readable for LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_aonclkrst_hpcpu_reset_ctrl0::W`](W) writer structure"]
impl crate::Writable for LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets LP_AONCLKRST_HPCPU_RESET_CTRL0 to value 0x8002_8002"]
impl crate::Resettable for LP_AONCLKRST_HPCPU_RESET_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x8002_8002;
}
