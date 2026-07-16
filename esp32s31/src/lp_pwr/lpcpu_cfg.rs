#[doc = "Register `LPCPU_CFG` reader"]
pub type R = crate::R<LPCPU_CFG_SPEC>;
#[doc = "Register `LPCPU_CFG` writer"]
pub type W = crate::W<LPCPU_CFG_SPEC>;
#[doc = "Field `LPCPU_STALL_WAIT_TIMER` reader - stall wait timer for lpcpu during lpcpu power off sequence"]
pub type LPCPU_STALL_WAIT_TIMER_R = crate::FieldReader;
#[doc = "Field `LPCPU_STALL_WAIT_TIMER` writer - stall wait timer for lpcpu during lpcpu power off sequence"]
pub type LPCPU_STALL_WAIT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `LPCPU_STALL_FLAG_EN` reader - 1: wait for stall rdy input during lpcpu power off sequence 0:do not wait for stall rdy during lpcpu power off sequence"]
pub type LPCPU_STALL_FLAG_EN_R = crate::BitReader;
#[doc = "Field `LPCPU_STALL_FLAG_EN` writer - 1: wait for stall rdy input during lpcpu power off sequence 0:do not wait for stall rdy during lpcpu power off sequence"]
pub type LPCPU_STALL_FLAG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_STALL_EN` reader - 1: enable lpcpu stall during lpcpu power off sequence 0: disable lpcpu stall during lpcpu power off sequence"]
pub type LPCPU_STALL_EN_R = crate::BitReader;
#[doc = "Field `LPCPU_STALL_EN` writer - 1: enable lpcpu stall during lpcpu power off sequence 0: disable lpcpu stall during lpcpu power off sequence"]
pub type LPCPU_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_RESET_EN` reader - 1: enable lpcpu reset during lpcpu power off sequence 0: disable lpcpu reset during lpcpu power off sequence"]
pub type LPCPU_RESET_EN_R = crate::BitReader;
#[doc = "Field `LPCPU_RESET_EN` writer - 1: enable lpcpu reset during lpcpu power off sequence 0: disable lpcpu reset during lpcpu power off sequence"]
pub type LPCPU_RESET_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_SLP_WAITI_FLAG_EN` reader - 1: validates lp cpu waiti rdy signal during lpcpu power off sequence 0: invalidates lp cpu waiti rdy signal during lpcpu power off sequence"]
pub type LPCPU_SLP_WAITI_FLAG_EN_R = crate::BitReader;
#[doc = "Field `LPCPU_SLP_WAITI_FLAG_EN` writer - 1: validates lp cpu waiti rdy signal during lpcpu power off sequence 0: invalidates lp cpu waiti rdy signal during lpcpu power off sequence"]
pub type LPCPU_SLP_WAITI_FLAG_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LPCPU_SLP_BYPASS_INTR_EN` reader - 1: enable interrupt bypass signal for lpcpu during lpcpu power off sequence 0: disable interrupt bypass signal for lpcpu during lpcpu power off sequence"]
pub type LPCPU_SLP_BYPASS_INTR_EN_R = crate::BitReader;
#[doc = "Field `LPCPU_SLP_BYPASS_INTR_EN` writer - 1: enable interrupt bypass signal for lpcpu during lpcpu power off sequence 0: disable interrupt bypass signal for lpcpu during lpcpu power off sequence"]
pub type LPCPU_SLP_BYPASS_INTR_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7 - stall wait timer for lpcpu during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_stall_wait_timer(&self) -> LPCPU_STALL_WAIT_TIMER_R {
        LPCPU_STALL_WAIT_TIMER_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - 1: wait for stall rdy input during lpcpu power off sequence 0:do not wait for stall rdy during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_stall_flag_en(&self) -> LPCPU_STALL_FLAG_EN_R {
        LPCPU_STALL_FLAG_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - 1: enable lpcpu stall during lpcpu power off sequence 0: disable lpcpu stall during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_stall_en(&self) -> LPCPU_STALL_EN_R {
        LPCPU_STALL_EN_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - 1: enable lpcpu reset during lpcpu power off sequence 0: disable lpcpu reset during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_reset_en(&self) -> LPCPU_RESET_EN_R {
        LPCPU_RESET_EN_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - 1: validates lp cpu waiti rdy signal during lpcpu power off sequence 0: invalidates lp cpu waiti rdy signal during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_slp_waiti_flag_en(&self) -> LPCPU_SLP_WAITI_FLAG_EN_R {
        LPCPU_SLP_WAITI_FLAG_EN_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - 1: enable interrupt bypass signal for lpcpu during lpcpu power off sequence 0: disable interrupt bypass signal for lpcpu during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_slp_bypass_intr_en(&self) -> LPCPU_SLP_BYPASS_INTR_EN_R {
        LPCPU_SLP_BYPASS_INTR_EN_R::new(((self.bits >> 12) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LPCPU_CFG")
            .field("lpcpu_stall_wait_timer", &self.lpcpu_stall_wait_timer())
            .field("lpcpu_stall_flag_en", &self.lpcpu_stall_flag_en())
            .field("lpcpu_stall_en", &self.lpcpu_stall_en())
            .field("lpcpu_reset_en", &self.lpcpu_reset_en())
            .field("lpcpu_slp_waiti_flag_en", &self.lpcpu_slp_waiti_flag_en())
            .field("lpcpu_slp_bypass_intr_en", &self.lpcpu_slp_bypass_intr_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - stall wait timer for lpcpu during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_stall_wait_timer(&mut self) -> LPCPU_STALL_WAIT_TIMER_W<'_, LPCPU_CFG_SPEC> {
        LPCPU_STALL_WAIT_TIMER_W::new(self, 0)
    }
    #[doc = "Bit 8 - 1: wait for stall rdy input during lpcpu power off sequence 0:do not wait for stall rdy during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_stall_flag_en(&mut self) -> LPCPU_STALL_FLAG_EN_W<'_, LPCPU_CFG_SPEC> {
        LPCPU_STALL_FLAG_EN_W::new(self, 8)
    }
    #[doc = "Bit 9 - 1: enable lpcpu stall during lpcpu power off sequence 0: disable lpcpu stall during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_stall_en(&mut self) -> LPCPU_STALL_EN_W<'_, LPCPU_CFG_SPEC> {
        LPCPU_STALL_EN_W::new(self, 9)
    }
    #[doc = "Bit 10 - 1: enable lpcpu reset during lpcpu power off sequence 0: disable lpcpu reset during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_reset_en(&mut self) -> LPCPU_RESET_EN_W<'_, LPCPU_CFG_SPEC> {
        LPCPU_RESET_EN_W::new(self, 10)
    }
    #[doc = "Bit 11 - 1: validates lp cpu waiti rdy signal during lpcpu power off sequence 0: invalidates lp cpu waiti rdy signal during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_slp_waiti_flag_en(&mut self) -> LPCPU_SLP_WAITI_FLAG_EN_W<'_, LPCPU_CFG_SPEC> {
        LPCPU_SLP_WAITI_FLAG_EN_W::new(self, 11)
    }
    #[doc = "Bit 12 - 1: enable interrupt bypass signal for lpcpu during lpcpu power off sequence 0: disable interrupt bypass signal for lpcpu during lpcpu power off sequence"]
    #[inline(always)]
    pub fn lpcpu_slp_bypass_intr_en(&mut self) -> LPCPU_SLP_BYPASS_INTR_EN_W<'_, LPCPU_CFG_SPEC> {
        LPCPU_SLP_BYPASS_INTR_EN_W::new(self, 12)
    }
}
#[doc = "config register for lpcpu power control\n\nYou can [`read`](crate::Reg::read) this register and get [`lpcpu_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lpcpu_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LPCPU_CFG_SPEC;
impl crate::RegisterSpec for LPCPU_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lpcpu_cfg::R`](R) reader structure"]
impl crate::Readable for LPCPU_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lpcpu_cfg::W`](W) writer structure"]
impl crate::Writable for LPCPU_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LPCPU_CFG to value 0x0700"]
impl crate::Resettable for LPCPU_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0700;
}
