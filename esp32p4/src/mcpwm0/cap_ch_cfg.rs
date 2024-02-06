#[doc = "Register `CAP_CH%s_CFG` reader"]
pub type R = crate::R<CAP_CH_CFG_SPEC>;
#[doc = "Register `CAP_CH%s_CFG` writer"]
pub type W = crate::W<CAP_CH_CFG_SPEC>;
#[doc = "Field `CAP_EN` reader - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type CAP_EN_R = crate::BitReader;
#[doc = "Field `CAP_EN` writer - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type CAP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_MODE` reader - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
pub type CAP_MODE_R = crate::FieldReader;
#[doc = "Field `CAP_MODE` writer - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
pub type CAP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAP_PRESCALE` reader - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
pub type CAP_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CAP_PRESCALE` writer - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
pub type CAP_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAP_IN_INVERT` reader - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
pub type CAP_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CAP_IN_INVERT` writer - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
pub type CAP_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SW` writer - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
pub type CAP_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_en(&self) -> CAP_EN_R {
        CAP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
    #[inline(always)]
    pub fn cap_mode(&self) -> CAP_MODE_R {
        CAP_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap_prescale(&self) -> CAP_PRESCALE_R {
        CAP_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn cap_in_invert(&self) -> CAP_IN_INVERT_R {
        CAP_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH_CFG")
            .field("cap_en", &format_args!("{}", self.cap_en().bit()))
            .field("cap_mode", &format_args!("{}", self.cap_mode().bits()))
            .field(
                "cap_prescale",
                &format_args!("{}", self.cap_prescale().bits()),
            )
            .field(
                "cap_in_invert",
                &format_args!("{}", self.cap_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    #[must_use]
    pub fn cap_en(&mut self) -> CAP_EN_W<CAP_CH_CFG_SPEC> {
        CAP_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn cap_mode(&mut self) -> CAP_MODE_W<CAP_CH_CFG_SPEC> {
        CAP_MODE_W::new(self, 1)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    #[must_use]
    pub fn cap_prescale(&mut self) -> CAP_PRESCALE_W<CAP_CH_CFG_SPEC> {
        CAP_PRESCALE_W::new(self, 3)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    #[must_use]
    pub fn cap_in_invert(&mut self) -> CAP_IN_INVERT_W<CAP_CH_CFG_SPEC> {
        CAP_IN_INVERT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
    #[inline(always)]
    #[must_use]
    pub fn cap_sw(&mut self) -> CAP_SW_W<CAP_CH_CFG_SPEC> {
        CAP_SW_W::new(self, 12)
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
#[doc = "Capture channel %s configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CH_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch_cfg::R`](R) reader structure"]
impl crate::Readable for CAP_CH_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_ch_cfg::W`](W) writer structure"]
impl crate::Writable for CAP_CH_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_CH%s_CFG to value 0"]
impl crate::Resettable for CAP_CH_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
