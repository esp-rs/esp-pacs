#[doc = "Register `CAP_CH%s_CFG` reader"]
pub type R = crate::R<CAP_CH_CFG_SPEC>;
#[doc = "Register `CAP_CH%s_CFG` writer"]
pub type W = crate::W<CAP_CH_CFG_SPEC>;
#[doc = "Field `EN` reader - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Describes which edges to trigger a capture event\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum CAP_MODE {
    #[doc = "0: Capture nothing"]
    None = 0,
    #[doc = "1: Capture falling edges"]
    FallingEdge = 1,
    #[doc = "2: Capture rising edges"]
    RisingEdge = 2,
    #[doc = "3: Capture any edges"]
    AnyEdge = 3,
}
impl From<CAP_MODE> for u8 {
    #[inline(always)]
    fn from(variant: CAP_MODE) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for CAP_MODE {
    type Ux = u8;
}
impl crate::IsEnum for CAP_MODE {}
#[doc = "Field `CAP_MODE` reader - Describes which edges to trigger a capture event"]
pub type CAP_MODE_R = crate::FieldReader<CAP_MODE>;
impl CAP_MODE_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> CAP_MODE {
        match self.bits {
            0 => CAP_MODE::None,
            1 => CAP_MODE::FallingEdge,
            2 => CAP_MODE::RisingEdge,
            3 => CAP_MODE::AnyEdge,
            _ => unreachable!(),
        }
    }
    #[doc = "Capture nothing"]
    #[inline(always)]
    pub fn is_none(&self) -> bool {
        *self == CAP_MODE::None
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn is_falling_edge(&self) -> bool {
        *self == CAP_MODE::FallingEdge
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn is_rising_edge(&self) -> bool {
        *self == CAP_MODE::RisingEdge
    }
    #[doc = "Capture any edges"]
    #[inline(always)]
    pub fn is_any_edge(&self) -> bool {
        *self == CAP_MODE::AnyEdge
    }
}
#[doc = "Field `CAP_MODE` writer - Describes which edges to trigger a capture event"]
pub type CAP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2, CAP_MODE, crate::Safe>;
impl<'a, REG> CAP_MODE_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "Capture nothing"]
    #[inline(always)]
    pub fn none(self) -> &'a mut crate::W<REG> {
        self.variant(CAP_MODE::None)
    }
    #[doc = "Capture falling edges"]
    #[inline(always)]
    pub fn falling_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP_MODE::FallingEdge)
    }
    #[doc = "Capture rising edges"]
    #[inline(always)]
    pub fn rising_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP_MODE::RisingEdge)
    }
    #[doc = "Capture any edges"]
    #[inline(always)]
    pub fn any_edge(self) -> &'a mut crate::W<REG> {
        self.variant(CAP_MODE::AnyEdge)
    }
}
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
    #[doc = "Bits 1:2 - Describes which edges to trigger a capture event"]
    #[inline(always)]
    pub fn cap_mode(&self) -> CAP_MODE_R {
        CAP_MODE_R::new(((self.bits >> 1) & 3) as u8)
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
            .field("en", &self.en())
            .field("cap_mode", &self.cap_mode())
            .field("prescale", &self.prescale())
            .field("in_invert", &self.in_invert())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable capture on channel %s.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn en(&mut self) -> EN_W<'_, CAP_CH_CFG_SPEC> {
        EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Describes which edges to trigger a capture event"]
    #[inline(always)]
    pub fn cap_mode(&mut self) -> CAP_MODE_W<'_, CAP_CH_CFG_SPEC> {
        CAP_MODE_W::new(self, 1)
    }
    #[doc = "Bits 3:10 - Configures prescale value on possitive edge of CAP%s. Prescale value = PWM_CAP%s_PRESCALE + 1"]
    #[inline(always)]
    pub fn prescale(&mut self) -> PRESCALE_W<'_, CAP_CH_CFG_SPEC> {
        PRESCALE_W::new(self, 3)
    }
    #[doc = "Bit 11 - Configures whether or not to invert CAP%s from GPIO matrix before prescale.\\\\0: Normal\\\\1: Invert"]
    #[inline(always)]
    pub fn in_invert(&mut self) -> IN_INVERT_W<'_, CAP_CH_CFG_SPEC> {
        IN_INVERT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Configures the generation of software capture.\\\\0: Invalid, No effect\\\\1: Trigger a software forced capture on channel %s"]
    #[inline(always)]
    pub fn sw(&mut self) -> SW_W<'_, CAP_CH_CFG_SPEC> {
        SW_W::new(self, 12)
    }
}
#[doc = "Capture channel %s configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_ch_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_ch_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CH_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch_cfg::R`](R) reader structure"]
impl crate::Readable for CAP_CH_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_ch_cfg::W`](W) writer structure"]
impl crate::Writable for CAP_CH_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAP_CH%s_CFG to value 0"]
impl crate::Resettable for CAP_CH_CFG_SPEC {}
