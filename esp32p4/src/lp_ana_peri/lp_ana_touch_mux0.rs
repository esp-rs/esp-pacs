#[doc = "Register `LP_ANA_TOUCH_MUX0` reader"]
pub type R = crate::R<LP_ANA_TOUCH_MUX0_SPEC>;
#[doc = "Register `LP_ANA_TOUCH_MUX0` writer"]
pub type W = crate::W<LP_ANA_TOUCH_MUX0_SPEC>;
#[doc = "Field `LP_ANA_TOUCH_DATA_SEL` reader - need_des"]
pub type LP_ANA_TOUCH_DATA_SEL_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_DATA_SEL` writer - need_des"]
pub type LP_ANA_TOUCH_DATA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ANA_TOUCH_FREQ_SEL` reader - need_des"]
pub type LP_ANA_TOUCH_FREQ_SEL_R = crate::FieldReader;
#[doc = "Field `LP_ANA_TOUCH_FREQ_SEL` writer - need_des"]
pub type LP_ANA_TOUCH_FREQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `LP_ANA_TOUCH_BUFSEL` reader - need_des"]
pub type LP_ANA_TOUCH_BUFSEL_R = crate::FieldReader<u16>;
#[doc = "Field `LP_ANA_TOUCH_BUFSEL` writer - need_des"]
pub type LP_ANA_TOUCH_BUFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `LP_ANA_TOUCH_DONE_EN` reader - need_des"]
pub type LP_ANA_TOUCH_DONE_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_DONE_EN` writer - need_des"]
pub type LP_ANA_TOUCH_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_DONE_FORCE` reader - need_des"]
pub type LP_ANA_TOUCH_DONE_FORCE_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_DONE_FORCE` writer - need_des"]
pub type LP_ANA_TOUCH_DONE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_FSM_EN` reader - need_des"]
pub type LP_ANA_TOUCH_FSM_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_FSM_EN` writer - need_des"]
pub type LP_ANA_TOUCH_FSM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_START_EN` reader - need_des"]
pub type LP_ANA_TOUCH_START_EN_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_START_EN` writer - need_des"]
pub type LP_ANA_TOUCH_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_ANA_TOUCH_START_FORCE` reader - need_des"]
pub type LP_ANA_TOUCH_START_FORCE_R = crate::BitReader;
#[doc = "Field `LP_ANA_TOUCH_START_FORCE` writer - need_des"]
pub type LP_ANA_TOUCH_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_data_sel(&self) -> LP_ANA_TOUCH_DATA_SEL_R {
        LP_ANA_TOUCH_DATA_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_freq_sel(&self) -> LP_ANA_TOUCH_FREQ_SEL_R {
        LP_ANA_TOUCH_FREQ_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:26 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_bufsel(&self) -> LP_ANA_TOUCH_BUFSEL_R {
        LP_ANA_TOUCH_BUFSEL_R::new(((self.bits >> 12) & 0x7fff) as u16)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_done_en(&self) -> LP_ANA_TOUCH_DONE_EN_R {
        LP_ANA_TOUCH_DONE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_done_force(&self) -> LP_ANA_TOUCH_DONE_FORCE_R {
        LP_ANA_TOUCH_DONE_FORCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_fsm_en(&self) -> LP_ANA_TOUCH_FSM_EN_R {
        LP_ANA_TOUCH_FSM_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_start_en(&self) -> LP_ANA_TOUCH_START_EN_R {
        LP_ANA_TOUCH_START_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn lp_ana_touch_start_force(&self) -> LP_ANA_TOUCH_START_FORCE_R {
        LP_ANA_TOUCH_START_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ANA_TOUCH_MUX0")
            .field(
                "lp_ana_touch_data_sel",
                &format_args!("{}", self.lp_ana_touch_data_sel().bits()),
            )
            .field(
                "lp_ana_touch_freq_sel",
                &format_args!("{}", self.lp_ana_touch_freq_sel().bits()),
            )
            .field(
                "lp_ana_touch_bufsel",
                &format_args!("{}", self.lp_ana_touch_bufsel().bits()),
            )
            .field(
                "lp_ana_touch_done_en",
                &format_args!("{}", self.lp_ana_touch_done_en().bit()),
            )
            .field(
                "lp_ana_touch_done_force",
                &format_args!("{}", self.lp_ana_touch_done_force().bit()),
            )
            .field(
                "lp_ana_touch_fsm_en",
                &format_args!("{}", self.lp_ana_touch_fsm_en().bit()),
            )
            .field(
                "lp_ana_touch_start_en",
                &format_args!("{}", self.lp_ana_touch_start_en().bit()),
            )
            .field(
                "lp_ana_touch_start_force",
                &format_args!("{}", self.lp_ana_touch_start_force().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ANA_TOUCH_MUX0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 8:9 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_data_sel(&mut self) -> LP_ANA_TOUCH_DATA_SEL_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_DATA_SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_freq_sel(&mut self) -> LP_ANA_TOUCH_FREQ_SEL_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_FREQ_SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:26 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_bufsel(&mut self) -> LP_ANA_TOUCH_BUFSEL_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_BUFSEL_W::new(self, 12)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_done_en(&mut self) -> LP_ANA_TOUCH_DONE_EN_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_DONE_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_done_force(&mut self) -> LP_ANA_TOUCH_DONE_FORCE_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_DONE_FORCE_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_fsm_en(&mut self) -> LP_ANA_TOUCH_FSM_EN_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_FSM_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_start_en(&mut self) -> LP_ANA_TOUCH_START_EN_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_START_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_ana_touch_start_force(
        &mut self,
    ) -> LP_ANA_TOUCH_START_FORCE_W<LP_ANA_TOUCH_MUX0_SPEC> {
        LP_ANA_TOUCH_START_FORCE_W::new(self, 31)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_ana_touch_mux0::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lp_ana_touch_mux0::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ANA_TOUCH_MUX0_SPEC;
impl crate::RegisterSpec for LP_ANA_TOUCH_MUX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_ana_touch_mux0::R`](R) reader structure"]
impl crate::Readable for LP_ANA_TOUCH_MUX0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`lp_ana_touch_mux0::W`](W) writer structure"]
impl crate::Writable for LP_ANA_TOUCH_MUX0_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets LP_ANA_TOUCH_MUX0 to value 0x2000_0000"]
impl crate::Resettable for LP_ANA_TOUCH_MUX0_SPEC {
    const RESET_VALUE: Self::Ux = 0x2000_0000;
}
