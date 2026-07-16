#[doc = "Register `SLP_WAKEUP_CNTL8` reader"]
pub type R = crate::R<SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "Register `SLP_WAKEUP_CNTL8` writer"]
pub type W = crate::W<SLP_WAKEUP_CNTL8_SPEC>;
#[doc = "Field `PD_TOP_PU_DELT_TIMER` reader - need_des"]
pub type PD_TOP_PU_DELT_TIMER_R = crate::FieldReader;
#[doc = "Field `PD_TOP_PU_DELT_TIMER` writer - need_des"]
pub type PD_TOP_PU_DELT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD_HP_CPU_PU_DELT_TIMER` reader - need_des"]
pub type PD_HP_CPU_PU_DELT_TIMER_R = crate::FieldReader;
#[doc = "Field `PD_HP_CPU_PU_DELT_TIMER` writer - need_des"]
pub type PD_HP_CPU_PU_DELT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD_MODEM_TOP_PU_DELT_TIMER` reader - need_des"]
pub type PD_MODEM_TOP_PU_DELT_TIMER_R = crate::FieldReader;
#[doc = "Field `PD_MODEM_TOP_PU_DELT_TIMER` writer - need_des"]
pub type PD_MODEM_TOP_PU_DELT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD_MODEM_PWR_PU_DELT_TIMER` reader - need_des"]
pub type PD_MODEM_PWR_PU_DELT_TIMER_R = crate::FieldReader;
#[doc = "Field `PD_MODEM_PWR_PU_DELT_TIMER` writer - need_des"]
pub type PD_MODEM_PWR_PU_DELT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD_HP_CNNT_PU_DELT_TIMER` reader - need_des"]
pub type PD_HP_CNNT_PU_DELT_TIMER_R = crate::FieldReader;
#[doc = "Field `PD_HP_CNNT_PU_DELT_TIMER` writer - need_des"]
pub type PD_HP_CNNT_PU_DELT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `PD_HP_ALIVE_PU_DELT_TIMER` reader - need_des"]
pub type PD_HP_ALIVE_PU_DELT_TIMER_R = crate::FieldReader;
#[doc = "Field `PD_HP_ALIVE_PU_DELT_TIMER` writer - need_des"]
pub type PD_HP_ALIVE_PU_DELT_TIMER_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn pd_top_pu_delt_timer(&self) -> PD_TOP_PU_DELT_TIMER_R {
        PD_TOP_PU_DELT_TIMER_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cpu_pu_delt_timer(&self) -> PD_HP_CPU_PU_DELT_TIMER_R {
        PD_HP_CPU_PU_DELT_TIMER_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - need_des"]
    #[inline(always)]
    pub fn pd_modem_top_pu_delt_timer(&self) -> PD_MODEM_TOP_PU_DELT_TIMER_R {
        PD_MODEM_TOP_PU_DELT_TIMER_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - need_des"]
    #[inline(always)]
    pub fn pd_modem_pwr_pu_delt_timer(&self) -> PD_MODEM_PWR_PU_DELT_TIMER_R {
        PD_MODEM_PWR_PU_DELT_TIMER_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cnnt_pu_delt_timer(&self) -> PD_HP_CNNT_PU_DELT_TIMER_R {
        PD_HP_CNNT_PU_DELT_TIMER_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - need_des"]
    #[inline(always)]
    pub fn pd_hp_alive_pu_delt_timer(&self) -> PD_HP_ALIVE_PU_DELT_TIMER_R {
        PD_HP_ALIVE_PU_DELT_TIMER_R::new(((self.bits >> 15) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLP_WAKEUP_CNTL8")
            .field("pd_top_pu_delt_timer", &self.pd_top_pu_delt_timer())
            .field("pd_hp_cpu_pu_delt_timer", &self.pd_hp_cpu_pu_delt_timer())
            .field(
                "pd_modem_top_pu_delt_timer",
                &self.pd_modem_top_pu_delt_timer(),
            )
            .field(
                "pd_modem_pwr_pu_delt_timer",
                &self.pd_modem_pwr_pu_delt_timer(),
            )
            .field("pd_hp_cnnt_pu_delt_timer", &self.pd_hp_cnnt_pu_delt_timer())
            .field(
                "pd_hp_alive_pu_delt_timer",
                &self.pd_hp_alive_pu_delt_timer(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - need_des"]
    #[inline(always)]
    pub fn pd_top_pu_delt_timer(&mut self) -> PD_TOP_PU_DELT_TIMER_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        PD_TOP_PU_DELT_TIMER_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cpu_pu_delt_timer(
        &mut self,
    ) -> PD_HP_CPU_PU_DELT_TIMER_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        PD_HP_CPU_PU_DELT_TIMER_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - need_des"]
    #[inline(always)]
    pub fn pd_modem_top_pu_delt_timer(
        &mut self,
    ) -> PD_MODEM_TOP_PU_DELT_TIMER_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        PD_MODEM_TOP_PU_DELT_TIMER_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - need_des"]
    #[inline(always)]
    pub fn pd_modem_pwr_pu_delt_timer(
        &mut self,
    ) -> PD_MODEM_PWR_PU_DELT_TIMER_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        PD_MODEM_PWR_PU_DELT_TIMER_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - need_des"]
    #[inline(always)]
    pub fn pd_hp_cnnt_pu_delt_timer(
        &mut self,
    ) -> PD_HP_CNNT_PU_DELT_TIMER_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        PD_HP_CNNT_PU_DELT_TIMER_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - need_des"]
    #[inline(always)]
    pub fn pd_hp_alive_pu_delt_timer(
        &mut self,
    ) -> PD_HP_ALIVE_PU_DELT_TIMER_W<'_, SLP_WAKEUP_CNTL8_SPEC> {
        PD_HP_ALIVE_PU_DELT_TIMER_W::new(self, 15)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`slp_wakeup_cntl8::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`slp_wakeup_cntl8::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SLP_WAKEUP_CNTL8_SPEC;
impl crate::RegisterSpec for SLP_WAKEUP_CNTL8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`slp_wakeup_cntl8::R`](R) reader structure"]
impl crate::Readable for SLP_WAKEUP_CNTL8_SPEC {}
#[doc = "`write(|w| ..)` method takes [`slp_wakeup_cntl8::W`](W) writer structure"]
impl crate::Writable for SLP_WAKEUP_CNTL8_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SLP_WAKEUP_CNTL8 to value 0"]
impl crate::Resettable for SLP_WAKEUP_CNTL8_SPEC {}
