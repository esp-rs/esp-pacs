#[doc = "Register `CAP_TIMER_CFG` reader"]
pub type R = crate::R<CAP_TIMER_CFG_SPEC>;
#[doc = "Register `CAP_TIMER_CFG` writer"]
pub type W = crate::W<CAP_TIMER_CFG_SPEC>;
#[doc = "Field `CAP_TIMER_EN` reader - When set, capture timer incrementing under APB_clk is enabled."]
pub type CAP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CAP_TIMER_EN` writer - When set, capture timer incrementing under APB_clk is enabled."]
pub type CAP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SYNCI_EN` reader - When set, capture timer sync is enabled."]
pub type CAP_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `CAP_SYNCI_EN` writer - When set, capture timer sync is enabled."]
pub type CAP_SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SYNCI_SEL` reader - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
pub type CAP_SYNCI_SEL_R = crate::FieldReader;
#[doc = "Field `CAP_SYNCI_SEL` writer - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
pub type CAP_SYNCI_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAP_SYNC_SW` writer - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register."]
pub type CAP_SYNC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn cap_timer_en(&self) -> CAP_TIMER_EN_R {
        CAP_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, capture timer sync is enabled."]
    #[inline(always)]
    pub fn cap_synci_en(&self) -> CAP_SYNCI_EN_R {
        CAP_SYNCI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn cap_synci_sel(&self) -> CAP_SYNCI_SEL_R {
        CAP_SYNCI_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_CFG")
            .field(
                "cap_timer_en",
                &format_args!("{}", self.cap_timer_en().bit()),
            )
            .field(
                "cap_synci_en",
                &format_args!("{}", self.cap_synci_en().bit()),
            )
            .field(
                "cap_synci_sel",
                &format_args!("{}", self.cap_synci_sel().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CAP_TIMER_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap_timer_en(&mut self) -> CAP_TIMER_EN_W<CAP_TIMER_CFG_SPEC> {
        CAP_TIMER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - When set, capture timer sync is enabled."]
    #[inline(always)]
    #[must_use]
    pub fn cap_synci_en(&mut self) -> CAP_SYNCI_EN_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNCI_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    #[must_use]
    pub fn cap_synci_sel(&mut self) -> CAP_SYNCI_SEL_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNCI_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register."]
    #[inline(always)]
    #[must_use]
    pub fn cap_sync_sw(&mut self) -> CAP_SYNC_SW_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNC_SW_W::new(self, 5)
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
#[doc = "Configure capture timer\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_TIMER_CFG_SPEC;
impl crate::RegisterSpec for CAP_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_cfg::R`](R) reader structure"]
impl crate::Readable for CAP_TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_cfg::W`](W) writer structure"]
impl crate::Writable for CAP_TIMER_CFG_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CAP_TIMER_CFG to value 0"]
impl crate::Resettable for CAP_TIMER_CFG_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
