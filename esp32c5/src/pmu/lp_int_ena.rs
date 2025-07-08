#[doc = "Register `LP_INT_ENA` reader"]
pub type R = crate::R<LP_INT_ENA_SPEC>;
#[doc = "Register `LP_INT_ENA` writer"]
pub type W = crate::W<LP_INT_ENA_SPEC>;
#[doc = "Field `LP_CPU_WAKEUP_INT_ENA` reader - need_des"]
pub type LP_CPU_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_CPU_WAKEUP_INT_ENA` writer - need_des"]
pub type LP_CPU_WAKEUP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_ACTIVE_END_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_END_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_MODEM_END_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_SLEEP_END_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_ENA` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_END_INT_ENA` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_ACTIVE_START_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_ACTIVE_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_ACTIVE_START_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_ACTIVE_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_ENA` reader - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `SLEEP_SWITCH_MODEM_START_INT_ENA` writer - need_des"]
pub type SLEEP_SWITCH_MODEM_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_ENA` reader - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `MODEM_SWITCH_SLEEP_START_INT_ENA` writer - need_des"]
pub type MODEM_SWITCH_SLEEP_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_ENA` reader - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_ENA_R = crate::BitReader;
#[doc = "Field `ACTIVE_SWITCH_SLEEP_START_INT_ENA` writer - need_des"]
pub type ACTIVE_SWITCH_SLEEP_START_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_SW_TRIGGER_INT_ENA` reader - need_des"]
pub type HP_SW_TRIGGER_INT_ENA_R = crate::BitReader;
#[doc = "Field `HP_SW_TRIGGER_INT_ENA` writer - need_des"]
pub type HP_SW_TRIGGER_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_int_ena(&self) -> LP_CPU_WAKEUP_INT_ENA_R {
        LP_CPU_WAKEUP_INT_ENA_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_end_int_ena(&self) -> MODEM_SWITCH_ACTIVE_END_INT_ENA_R {
        MODEM_SWITCH_ACTIVE_END_INT_ENA_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_end_int_ena(&self) -> SLEEP_SWITCH_ACTIVE_END_INT_ENA_R {
        SLEEP_SWITCH_ACTIVE_END_INT_ENA_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_end_int_ena(&self) -> SLEEP_SWITCH_MODEM_END_INT_ENA_R {
        SLEEP_SWITCH_MODEM_END_INT_ENA_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_end_int_ena(&self) -> MODEM_SWITCH_SLEEP_END_INT_ENA_R {
        MODEM_SWITCH_SLEEP_END_INT_ENA_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_end_int_ena(&self) -> ACTIVE_SWITCH_SLEEP_END_INT_ENA_R {
        ACTIVE_SWITCH_SLEEP_END_INT_ENA_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_start_int_ena(&self) -> MODEM_SWITCH_ACTIVE_START_INT_ENA_R {
        MODEM_SWITCH_ACTIVE_START_INT_ENA_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_start_int_ena(&self) -> SLEEP_SWITCH_ACTIVE_START_INT_ENA_R {
        SLEEP_SWITCH_ACTIVE_START_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_start_int_ena(&self) -> SLEEP_SWITCH_MODEM_START_INT_ENA_R {
        SLEEP_SWITCH_MODEM_START_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_start_int_ena(&self) -> MODEM_SWITCH_SLEEP_START_INT_ENA_R {
        MODEM_SWITCH_SLEEP_START_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_start_int_ena(&self) -> ACTIVE_SWITCH_SLEEP_START_INT_ENA_R {
        ACTIVE_SWITCH_SLEEP_START_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sw_trigger_int_ena(&self) -> HP_SW_TRIGGER_INT_ENA_R {
        HP_SW_TRIGGER_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_INT_ENA")
            .field("lp_cpu_wakeup_int_ena", &self.lp_cpu_wakeup_int_ena())
            .field(
                "modem_switch_active_end_int_ena",
                &self.modem_switch_active_end_int_ena(),
            )
            .field(
                "sleep_switch_active_end_int_ena",
                &self.sleep_switch_active_end_int_ena(),
            )
            .field(
                "sleep_switch_modem_end_int_ena",
                &self.sleep_switch_modem_end_int_ena(),
            )
            .field(
                "modem_switch_sleep_end_int_ena",
                &self.modem_switch_sleep_end_int_ena(),
            )
            .field(
                "active_switch_sleep_end_int_ena",
                &self.active_switch_sleep_end_int_ena(),
            )
            .field(
                "modem_switch_active_start_int_ena",
                &self.modem_switch_active_start_int_ena(),
            )
            .field(
                "sleep_switch_active_start_int_ena",
                &self.sleep_switch_active_start_int_ena(),
            )
            .field(
                "sleep_switch_modem_start_int_ena",
                &self.sleep_switch_modem_start_int_ena(),
            )
            .field(
                "modem_switch_sleep_start_int_ena",
                &self.modem_switch_sleep_start_int_ena(),
            )
            .field(
                "active_switch_sleep_start_int_ena",
                &self.active_switch_sleep_start_int_ena(),
            )
            .field("hp_sw_trigger_int_ena", &self.hp_sw_trigger_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 20 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_wakeup_int_ena(&mut self) -> LP_CPU_WAKEUP_INT_ENA_W<LP_INT_ENA_SPEC> {
        LP_CPU_WAKEUP_INT_ENA_W::new(self, 20)
    }
    #[doc = "Bit 21 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_end_int_ena(
        &mut self,
    ) -> MODEM_SWITCH_ACTIVE_END_INT_ENA_W<LP_INT_ENA_SPEC> {
        MODEM_SWITCH_ACTIVE_END_INT_ENA_W::new(self, 21)
    }
    #[doc = "Bit 22 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_end_int_ena(
        &mut self,
    ) -> SLEEP_SWITCH_ACTIVE_END_INT_ENA_W<LP_INT_ENA_SPEC> {
        SLEEP_SWITCH_ACTIVE_END_INT_ENA_W::new(self, 22)
    }
    #[doc = "Bit 23 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_end_int_ena(
        &mut self,
    ) -> SLEEP_SWITCH_MODEM_END_INT_ENA_W<LP_INT_ENA_SPEC> {
        SLEEP_SWITCH_MODEM_END_INT_ENA_W::new(self, 23)
    }
    #[doc = "Bit 24 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_end_int_ena(
        &mut self,
    ) -> MODEM_SWITCH_SLEEP_END_INT_ENA_W<LP_INT_ENA_SPEC> {
        MODEM_SWITCH_SLEEP_END_INT_ENA_W::new(self, 24)
    }
    #[doc = "Bit 25 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_end_int_ena(
        &mut self,
    ) -> ACTIVE_SWITCH_SLEEP_END_INT_ENA_W<LP_INT_ENA_SPEC> {
        ACTIVE_SWITCH_SLEEP_END_INT_ENA_W::new(self, 25)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn modem_switch_active_start_int_ena(
        &mut self,
    ) -> MODEM_SWITCH_ACTIVE_START_INT_ENA_W<LP_INT_ENA_SPEC> {
        MODEM_SWITCH_ACTIVE_START_INT_ENA_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_active_start_int_ena(
        &mut self,
    ) -> SLEEP_SWITCH_ACTIVE_START_INT_ENA_W<LP_INT_ENA_SPEC> {
        SLEEP_SWITCH_ACTIVE_START_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sleep_switch_modem_start_int_ena(
        &mut self,
    ) -> SLEEP_SWITCH_MODEM_START_INT_ENA_W<LP_INT_ENA_SPEC> {
        SLEEP_SWITCH_MODEM_START_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn modem_switch_sleep_start_int_ena(
        &mut self,
    ) -> MODEM_SWITCH_SLEEP_START_INT_ENA_W<LP_INT_ENA_SPEC> {
        MODEM_SWITCH_SLEEP_START_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn active_switch_sleep_start_int_ena(
        &mut self,
    ) -> ACTIVE_SWITCH_SLEEP_START_INT_ENA_W<LP_INT_ENA_SPEC> {
        ACTIVE_SWITCH_SLEEP_START_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hp_sw_trigger_int_ena(&mut self) -> HP_SW_TRIGGER_INT_ENA_W<LP_INT_ENA_SPEC> {
        HP_SW_TRIGGER_INT_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`lp_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`lp_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_INT_ENA_SPEC;
impl crate::RegisterSpec for LP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_int_ena::R`](R) reader structure"]
impl crate::Readable for LP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_int_ena::W`](W) writer structure"]
impl crate::Writable for LP_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets LP_INT_ENA to value 0"]
impl crate::Resettable for LP_INT_ENA_SPEC {}
