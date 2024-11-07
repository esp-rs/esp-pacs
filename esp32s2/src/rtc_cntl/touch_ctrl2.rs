#[doc = "Register `TOUCH_CTRL2` reader"]
pub type R = crate::R<TOUCH_CTRL2_SPEC>;
#[doc = "Register `TOUCH_CTRL2` writer"]
pub type W = crate::W<TOUCH_CTRL2_SPEC>;
#[doc = "Field `TOUCH_DRANGE` reader - TOUCH attenuation."]
pub type TOUCH_DRANGE_R = crate::FieldReader;
#[doc = "Field `TOUCH_DRANGE` writer - TOUCH attenuation."]
pub type TOUCH_DRANGE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DREFL` reader - TOUCH reference voltage low. 0: 0.5 V 1: 0.6 V 2: 0.7 V 3: 0.8 V."]
pub type TOUCH_DREFL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFL` writer - TOUCH reference voltage low. 0: 0.5 V 1: 0.6 V 2: 0.7 V 3: 0.8 V."]
pub type TOUCH_DREFL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_DREFH` reader - TOUCH reference voltage high. 0: 2.4 V 1: 2.5 V 2: 2.6 V 3: 2.7 V."]
pub type TOUCH_DREFH_R = crate::FieldReader;
#[doc = "Field `TOUCH_DREFH` writer - TOUCH reference voltage high. 0: 2.4 V 1: 2.5 V 2: 2.6 V 3: 2.7 V."]
pub type TOUCH_DREFH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_XPD_BIAS` reader - TOUCH BIAS power switch."]
pub type TOUCH_XPD_BIAS_R = crate::BitReader;
#[doc = "Field `TOUCH_XPD_BIAS` writer - TOUCH BIAS power switch."]
pub type TOUCH_XPD_BIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_REFC` reader - Touch pad 0 reference capacitance."]
pub type TOUCH_REFC_R = crate::FieldReader;
#[doc = "Field `TOUCH_REFC` writer - Touch pad 0 reference capacitance."]
pub type TOUCH_REFC_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `TOUCH_DBIAS` reader - 0: Use bandgap bias. 1: Use self bias."]
pub type TOUCH_DBIAS_R = crate::BitReader;
#[doc = "Field `TOUCH_DBIAS` writer - 0: Use bandgap bias. 1: Use self bias."]
pub type TOUCH_DBIAS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_SLP_TIMER_EN` reader - Touch timer enable bit."]
pub type TOUCH_SLP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_SLP_TIMER_EN` writer - Touch timer enable bit."]
pub type TOUCH_SLP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_FSM_EN` reader - 0: TOUCH_START and TOUCH_XPD are controlled by soft- ware. 1: TOUCH_START and TOUCH_XPD are controlled by the Touch FSM."]
pub type TOUCH_START_FSM_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FSM_EN` writer - 0: TOUCH_START and TOUCH_XPD are controlled by soft- ware. 1: TOUCH_START and TOUCH_XPD are controlled by the Touch FSM."]
pub type TOUCH_START_FSM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_EN` reader - 1: Start the Touch FSM, only valid when RTC_CNTL_TOUCH_START_FORCE = 1."]
pub type TOUCH_START_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_EN` writer - 1: Start the Touch FSM, only valid when RTC_CNTL_TOUCH_START_FORCE = 1."]
pub type TOUCH_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_FORCE` reader - 0: Start the Touch FSM by timer. 1: Start Touch FSM by software."]
pub type TOUCH_START_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FORCE` writer - 0: Start the Touch FSM by timer. 1: Start Touch FSM by software."]
pub type TOUCH_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_XPD_WAIT` reader - The waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD."]
pub type TOUCH_XPD_WAIT_R = crate::FieldReader;
#[doc = "Field `TOUCH_XPD_WAIT` writer - The waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD."]
pub type TOUCH_XPD_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `TOUCH_SLP_CYC_DIV` reader - When a touch pad is active, sleep cycle could be divided by this number."]
pub type TOUCH_SLP_CYC_DIV_R = crate::FieldReader;
#[doc = "Field `TOUCH_SLP_CYC_DIV` writer - When a touch pad is active, sleep cycle could be divided by this number."]
pub type TOUCH_SLP_CYC_DIV_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_TIMER_FORCE_DONE` reader - Force touch timer done."]
pub type TOUCH_TIMER_FORCE_DONE_R = crate::FieldReader;
#[doc = "Field `TOUCH_TIMER_FORCE_DONE` writer - Force touch timer done."]
pub type TOUCH_TIMER_FORCE_DONE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_RESET` reader - Reset TOUCH FSM via software."]
pub type TOUCH_RESET_R = crate::BitReader;
#[doc = "Field `TOUCH_RESET` writer - Reset TOUCH FSM via software."]
pub type TOUCH_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_CLK_FO` reader - Touch clock force on."]
pub type TOUCH_CLK_FO_R = crate::BitReader;
#[doc = "Field `TOUCH_CLK_FO` writer - Touch clock force on."]
pub type TOUCH_CLK_FO_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_CLKGATE_EN` reader - Touch clock enable bit."]
pub type TOUCH_CLKGATE_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_CLKGATE_EN` writer - Touch clock enable bit."]
pub type TOUCH_CLKGATE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 2:3 - TOUCH attenuation."]
    #[inline(always)]
    pub fn touch_drange(&self) -> TOUCH_DRANGE_R {
        TOUCH_DRANGE_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - TOUCH reference voltage low. 0: 0.5 V 1: 0.6 V 2: 0.7 V 3: 0.8 V."]
    #[inline(always)]
    pub fn touch_drefl(&self) -> TOUCH_DREFL_R {
        TOUCH_DREFL_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - TOUCH reference voltage high. 0: 2.4 V 1: 2.5 V 2: 2.6 V 3: 2.7 V."]
    #[inline(always)]
    pub fn touch_drefh(&self) -> TOUCH_DREFH_R {
        TOUCH_DREFH_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bit 8 - TOUCH BIAS power switch."]
    #[inline(always)]
    pub fn touch_xpd_bias(&self) -> TOUCH_XPD_BIAS_R {
        TOUCH_XPD_BIAS_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 9:11 - Touch pad 0 reference capacitance."]
    #[inline(always)]
    pub fn touch_refc(&self) -> TOUCH_REFC_R {
        TOUCH_REFC_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bit 12 - 0: Use bandgap bias. 1: Use self bias."]
    #[inline(always)]
    pub fn touch_dbias(&self) -> TOUCH_DBIAS_R {
        TOUCH_DBIAS_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Touch timer enable bit."]
    #[inline(always)]
    pub fn touch_slp_timer_en(&self) -> TOUCH_SLP_TIMER_EN_R {
        TOUCH_SLP_TIMER_EN_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - 0: TOUCH_START and TOUCH_XPD are controlled by soft- ware. 1: TOUCH_START and TOUCH_XPD are controlled by the Touch FSM."]
    #[inline(always)]
    pub fn touch_start_fsm_en(&self) -> TOUCH_START_FSM_EN_R {
        TOUCH_START_FSM_EN_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1: Start the Touch FSM, only valid when RTC_CNTL_TOUCH_START_FORCE = 1."]
    #[inline(always)]
    pub fn touch_start_en(&self) -> TOUCH_START_EN_R {
        TOUCH_START_EN_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - 0: Start the Touch FSM by timer. 1: Start Touch FSM by software."]
    #[inline(always)]
    pub fn touch_start_force(&self) -> TOUCH_START_FORCE_R {
        TOUCH_START_FORCE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bits 17:24 - The waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD."]
    #[inline(always)]
    pub fn touch_xpd_wait(&self) -> TOUCH_XPD_WAIT_R {
        TOUCH_XPD_WAIT_R::new(((self.bits >> 17) & 0xff) as u8)
    }
    #[doc = "Bits 25:26 - When a touch pad is active, sleep cycle could be divided by this number."]
    #[inline(always)]
    pub fn touch_slp_cyc_div(&self) -> TOUCH_SLP_CYC_DIV_R {
        TOUCH_SLP_CYC_DIV_R::new(((self.bits >> 25) & 3) as u8)
    }
    #[doc = "Bits 27:28 - Force touch timer done."]
    #[inline(always)]
    pub fn touch_timer_force_done(&self) -> TOUCH_TIMER_FORCE_DONE_R {
        TOUCH_TIMER_FORCE_DONE_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 29 - Reset TOUCH FSM via software."]
    #[inline(always)]
    pub fn touch_reset(&self) -> TOUCH_RESET_R {
        TOUCH_RESET_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Touch clock force on."]
    #[inline(always)]
    pub fn touch_clk_fo(&self) -> TOUCH_CLK_FO_R {
        TOUCH_CLK_FO_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Touch clock enable bit."]
    #[inline(always)]
    pub fn touch_clkgate_en(&self) -> TOUCH_CLKGATE_EN_R {
        TOUCH_CLKGATE_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_CTRL2")
            .field("touch_drange", &self.touch_drange())
            .field("touch_drefl", &self.touch_drefl())
            .field("touch_drefh", &self.touch_drefh())
            .field("touch_xpd_bias", &self.touch_xpd_bias())
            .field("touch_refc", &self.touch_refc())
            .field("touch_dbias", &self.touch_dbias())
            .field("touch_slp_timer_en", &self.touch_slp_timer_en())
            .field("touch_start_fsm_en", &self.touch_start_fsm_en())
            .field("touch_start_en", &self.touch_start_en())
            .field("touch_start_force", &self.touch_start_force())
            .field("touch_xpd_wait", &self.touch_xpd_wait())
            .field("touch_slp_cyc_div", &self.touch_slp_cyc_div())
            .field("touch_timer_force_done", &self.touch_timer_force_done())
            .field("touch_reset", &self.touch_reset())
            .field("touch_clk_fo", &self.touch_clk_fo())
            .field("touch_clkgate_en", &self.touch_clkgate_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:3 - TOUCH attenuation."]
    #[inline(always)]
    pub fn touch_drange(&mut self) -> TOUCH_DRANGE_W<TOUCH_CTRL2_SPEC> {
        TOUCH_DRANGE_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - TOUCH reference voltage low. 0: 0.5 V 1: 0.6 V 2: 0.7 V 3: 0.8 V."]
    #[inline(always)]
    pub fn touch_drefl(&mut self) -> TOUCH_DREFL_W<TOUCH_CTRL2_SPEC> {
        TOUCH_DREFL_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - TOUCH reference voltage high. 0: 2.4 V 1: 2.5 V 2: 2.6 V 3: 2.7 V."]
    #[inline(always)]
    pub fn touch_drefh(&mut self) -> TOUCH_DREFH_W<TOUCH_CTRL2_SPEC> {
        TOUCH_DREFH_W::new(self, 6)
    }
    #[doc = "Bit 8 - TOUCH BIAS power switch."]
    #[inline(always)]
    pub fn touch_xpd_bias(&mut self) -> TOUCH_XPD_BIAS_W<TOUCH_CTRL2_SPEC> {
        TOUCH_XPD_BIAS_W::new(self, 8)
    }
    #[doc = "Bits 9:11 - Touch pad 0 reference capacitance."]
    #[inline(always)]
    pub fn touch_refc(&mut self) -> TOUCH_REFC_W<TOUCH_CTRL2_SPEC> {
        TOUCH_REFC_W::new(self, 9)
    }
    #[doc = "Bit 12 - 0: Use bandgap bias. 1: Use self bias."]
    #[inline(always)]
    pub fn touch_dbias(&mut self) -> TOUCH_DBIAS_W<TOUCH_CTRL2_SPEC> {
        TOUCH_DBIAS_W::new(self, 12)
    }
    #[doc = "Bit 13 - Touch timer enable bit."]
    #[inline(always)]
    pub fn touch_slp_timer_en(&mut self) -> TOUCH_SLP_TIMER_EN_W<TOUCH_CTRL2_SPEC> {
        TOUCH_SLP_TIMER_EN_W::new(self, 13)
    }
    #[doc = "Bit 14 - 0: TOUCH_START and TOUCH_XPD are controlled by soft- ware. 1: TOUCH_START and TOUCH_XPD are controlled by the Touch FSM."]
    #[inline(always)]
    pub fn touch_start_fsm_en(&mut self) -> TOUCH_START_FSM_EN_W<TOUCH_CTRL2_SPEC> {
        TOUCH_START_FSM_EN_W::new(self, 14)
    }
    #[doc = "Bit 15 - 1: Start the Touch FSM, only valid when RTC_CNTL_TOUCH_START_FORCE = 1."]
    #[inline(always)]
    pub fn touch_start_en(&mut self) -> TOUCH_START_EN_W<TOUCH_CTRL2_SPEC> {
        TOUCH_START_EN_W::new(self, 15)
    }
    #[doc = "Bit 16 - 0: Start the Touch FSM by timer. 1: Start Touch FSM by software."]
    #[inline(always)]
    pub fn touch_start_force(&mut self) -> TOUCH_START_FORCE_W<TOUCH_CTRL2_SPEC> {
        TOUCH_START_FORCE_W::new(self, 16)
    }
    #[doc = "Bits 17:24 - The waiting cycles (in 8MHz) between TOUCH_START and TOUCH_XPD."]
    #[inline(always)]
    pub fn touch_xpd_wait(&mut self) -> TOUCH_XPD_WAIT_W<TOUCH_CTRL2_SPEC> {
        TOUCH_XPD_WAIT_W::new(self, 17)
    }
    #[doc = "Bits 25:26 - When a touch pad is active, sleep cycle could be divided by this number."]
    #[inline(always)]
    pub fn touch_slp_cyc_div(&mut self) -> TOUCH_SLP_CYC_DIV_W<TOUCH_CTRL2_SPEC> {
        TOUCH_SLP_CYC_DIV_W::new(self, 25)
    }
    #[doc = "Bits 27:28 - Force touch timer done."]
    #[inline(always)]
    pub fn touch_timer_force_done(&mut self) -> TOUCH_TIMER_FORCE_DONE_W<TOUCH_CTRL2_SPEC> {
        TOUCH_TIMER_FORCE_DONE_W::new(self, 27)
    }
    #[doc = "Bit 29 - Reset TOUCH FSM via software."]
    #[inline(always)]
    pub fn touch_reset(&mut self) -> TOUCH_RESET_W<TOUCH_CTRL2_SPEC> {
        TOUCH_RESET_W::new(self, 29)
    }
    #[doc = "Bit 30 - Touch clock force on."]
    #[inline(always)]
    pub fn touch_clk_fo(&mut self) -> TOUCH_CLK_FO_W<TOUCH_CTRL2_SPEC> {
        TOUCH_CLK_FO_W::new(self, 30)
    }
    #[doc = "Bit 31 - Touch clock enable bit."]
    #[inline(always)]
    pub fn touch_clkgate_en(&mut self) -> TOUCH_CLKGATE_EN_W<TOUCH_CTRL2_SPEC> {
        TOUCH_CLKGATE_EN_W::new(self, 31)
    }
}
#[doc = "Touch control register\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_CTRL2_SPEC;
impl crate::RegisterSpec for TOUCH_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_ctrl2::R`](R) reader structure"]
impl crate::Readable for TOUCH_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_ctrl2::W`](W) writer structure"]
impl crate::Writable for TOUCH_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TOUCH_CTRL2 to value 0x0008_40cc"]
impl crate::Resettable for TOUCH_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0x0008_40cc;
}
