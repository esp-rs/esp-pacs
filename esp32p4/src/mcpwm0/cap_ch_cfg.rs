#[doc = "Register `CAP_CH%s_CFG` reader"]
pub type R = crate::R<CAP_CH_CFG_SPEC>;
#[doc = "Register `CAP_CH%s_CFG` writer"]
pub type W = crate::W<CAP_CH_CFG_SPEC>;
#[doc = "Field `EN` reader - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODE` reader - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
pub type MODE_R = crate::FieldReader;
#[doc = "Field `MODE` writer - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
pub type MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PRESCALE` reader - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
pub type PRESCALE_R = crate::FieldReader;
#[doc = "Field `PRESCALE` writer - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
pub type PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `IN_INVERT` reader - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
pub type IN_INVERT_R = crate::BitReader;
#[doc = "Field `IN_INVERT` writer - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
pub type IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW` writer - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
pub type SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
    #[inline(always)]
    pub fn mode(&self) -> MODE_R {
        MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    pub fn prescale(&self) -> PRESCALE_R {
        PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&self) -> IN_INVERT_R {
        IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH_CFG")
            .field("en", &format_args!("{}", self.en().bit()))
            .field("mode", &format_args!("{}", self.mode().bits()))
            .field("prescale", &format_args!("{}", self.prescale().bits()))
            .field("in_invert", &format_args!("{}", self.in_invert().bit()))
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
    pub fn en(&mut self) -> EN_W<CAP_CH_CFG_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Configures which edge of capture on channel %s after prescaling is used.\\\\0: None\\\\Bit0 is set to 1: Rnable capture on the negative edge\\\\Bit1 is set to 1: Enable capture on the positive edge"]
    #[inline(always)]
    #[must_use]
    pub fn mode(&mut self) -> MODE_W<CAP_CH_CFG_SPEC> {
        MODE_W::new(self, 1)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    #[must_use]
    pub fn prescale(&mut self) -> PRESCALE_W<CAP_CH_CFG_SPEC> {
        PRESCALE_W::new(self, 3)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    #[must_use]
    pub fn in_invert(&mut self) -> IN_INVERT_W<CAP_CH_CFG_SPEC> {
        IN_INVERT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
    #[inline(always)]
    #[must_use]
    pub fn sw(&mut self) -> SW_W<CAP_CH_CFG_SPEC> {
        SW_W::new(self, 12)
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
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_CH%s_CFG to value 0"]
impl crate::Resettable for CAP_CH_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
