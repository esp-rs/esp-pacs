#[doc = "Register `CAP_CH2_CFG` reader"]
pub type R = crate::R<CAP_CH2_CFG_SPEC>;
#[doc = "Register `CAP_CH2_CFG` writer"]
pub type W = crate::W<CAP_CH2_CFG_SPEC>;
#[doc = "Field `CAP2_EN` reader - When set, capture on channel 2 is enabled"]
pub type CAP2_EN_R = crate::BitReader;
#[doc = "Field `CAP2_EN` writer - When set, capture on channel 2 is enabled"]
pub type CAP2_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2_MODE` reader - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP2_MODE_R = crate::FieldReader;
#[doc = "Field `CAP2_MODE` writer - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
pub type CAP2_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `CAP2_PRESCALE` reader - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
pub type CAP2_PRESCALE_R = crate::FieldReader;
#[doc = "Field `CAP2_PRESCALE` writer - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
pub type CAP2_PRESCALE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `CAP2_IN_INVERT` reader - when set, CAP2 form GPIO matrix is inverted before prescale"]
pub type CAP2_IN_INVERT_R = crate::BitReader;
#[doc = "Field `CAP2_IN_INVERT` writer - when set, CAP2 form GPIO matrix is inverted before prescale"]
pub type CAP2_IN_INVERT_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP2_SW` writer - Write 1 will trigger a software forced capture on channel 2"]
pub type CAP2_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    pub fn cap2_en(&self) -> CAP2_EN_R {
        CAP2_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    pub fn cap2_mode(&self) -> CAP2_MODE_R {
        CAP2_MODE_R::new(((self.bits >> 1) & 3) as u8)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    pub fn cap2_prescale(&self) -> CAP2_PRESCALE_R {
        CAP2_PRESCALE_R::new(((self.bits >> 3) & 0xff) as u8)
    }
    #[doc = "Bit 11 - when set, CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    pub fn cap2_in_invert(&self) -> CAP2_IN_INVERT_R {
        CAP2_IN_INVERT_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_CH2_CFG")
            .field("cap2_en", &format_args!("{}", self.cap2_en().bit()))
            .field("cap2_mode", &format_args!("{}", self.cap2_mode().bits()))
            .field(
                "cap2_prescale",
                &format_args!("{}", self.cap2_prescale().bits()),
            )
            .field(
                "cap2_in_invert",
                &format_args!("{}", self.cap2_in_invert().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_CH2_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture on channel 2 is enabled"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_en(&mut self) -> CAP2_EN_W<CAP_CH2_CFG_SPEC> {
        CAP2_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:2 - Edge of capture on channel 2 after prescaling. When bit0 is set to 1: enable capture on the negative edge, When bit1 is set to 1: enable capture on the positive edge."]
    #[inline(always)]
    #[must_use]
    pub fn cap2_mode(&mut self) -> CAP2_MODE_W<CAP_CH2_CFG_SPEC> {
        CAP2_MODE_W::new(self, 1)
    }
    #[doc = "Bits 3:10 - Value of prescaling on possitive edge of CAP2. Prescale value = PWM_CAP2_PRESCALE + 1"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_prescale(&mut self) -> CAP2_PRESCALE_W<CAP_CH2_CFG_SPEC> {
        CAP2_PRESCALE_W::new(self, 3)
    }
    #[doc = "Bit 11 - when set, CAP2 form GPIO matrix is inverted before prescale"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_in_invert(&mut self) -> CAP2_IN_INVERT_W<CAP_CH2_CFG_SPEC> {
        CAP2_IN_INVERT_W::new(self, 11)
    }
    #[doc = "Bit 12 - Write 1 will trigger a software forced capture on channel 2"]
    #[inline(always)]
    #[must_use]
    pub fn cap2_sw(&mut self) -> CAP2_SW_W<CAP_CH2_CFG_SPEC> {
        CAP2_SW_W::new(self, 12)
    }
}
#[doc = "Capture channel 2 configuration and enable\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_ch2_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_ch2_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_CH2_CFG_SPEC;
impl crate::RegisterSpec for CAP_CH2_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_ch2_cfg::R`](R) reader structure"]
impl crate::Readable for CAP_CH2_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_ch2_cfg::W`](W) writer structure"]
impl crate::Writable for CAP_CH2_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_CH2_CFG to value 0"]
impl crate::Resettable for CAP_CH2_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
