#[doc = "Register `ETM_EVENT_CH%s_CFG` reader"]
pub type R = crate::R<ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Register `ETM_EVENT_CH%s_CFG` writer"]
pub type W = crate::W<ETM_EVENT_CH_CFG_SPEC>;
#[doc = "Field `ETM_CH0_EVENT_SEL` reader - Etm event channel select gpio."]
pub type ETM_CH0_EVENT_SEL_R = crate::FieldReader;
#[doc = "Field `ETM_CH0_EVENT_SEL` writer - Etm event channel select gpio."]
pub type ETM_CH0_EVENT_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
#[doc = "Field `ETM_CH0_EVENT_EN` reader - Etm event send enable bit."]
pub type ETM_CH0_EVENT_EN_R = crate::BitReader;
#[doc = "Field `ETM_CH0_EVENT_EN` writer - Etm event send enable bit."]
pub type ETM_CH0_EVENT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:4 - Etm event channel select gpio."]
    #[inline(always)]
    pub fn etm_ch0_event_sel(&self) -> ETM_CH0_EVENT_SEL_R {
        ETM_CH0_EVENT_SEL_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 7 - Etm event send enable bit."]
    #[inline(always)]
    pub fn etm_ch0_event_en(&self) -> ETM_CH0_EVENT_EN_R {
        ETM_CH0_EVENT_EN_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_EVENT_CH_CFG")
            .field(
                "etm_ch0_event_sel",
                &format_args!("{}", self.etm_ch0_event_sel().bits()),
            )
            .field(
                "etm_ch0_event_en",
                &format_args!("{}", self.etm_ch0_event_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ETM_EVENT_CH_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:4 - Etm event channel select gpio."]
    #[inline(always)]
    #[must_use]
    pub fn etm_ch0_event_sel(&mut self) -> ETM_CH0_EVENT_SEL_W<ETM_EVENT_CH_CFG_SPEC> {
        ETM_CH0_EVENT_SEL_W::new(self, 0)
    }
    #[doc = "Bit 7 - Etm event send enable bit."]
    #[inline(always)]
    #[must_use]
    pub fn etm_ch0_event_en(&mut self) -> ETM_CH0_EVENT_EN_W<ETM_EVENT_CH_CFG_SPEC> {
        ETM_CH0_EVENT_EN_W::new(self, 7)
    }
}
#[doc = "Etm Config register of Channel%s\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`etm_event_ch_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`etm_event_ch_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_EVENT_CH_CFG_SPEC;
impl crate::RegisterSpec for ETM_EVENT_CH_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_event_ch_cfg::R`](R) reader structure"]
impl crate::Readable for ETM_EVENT_CH_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_event_ch_cfg::W`](W) writer structure"]
impl crate::Writable for ETM_EVENT_CH_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets ETM_EVENT_CH%s_CFG to value 0"]
impl crate::Resettable for ETM_EVENT_CH_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
