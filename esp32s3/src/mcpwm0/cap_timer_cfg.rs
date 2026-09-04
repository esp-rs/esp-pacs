#[doc = "Register `CAP_TIMER_CFG` reader"]
pub type R = crate::R<CAP_TIMER_CFG_SPEC>;
#[doc = "Register `CAP_TIMER_CFG` writer"]
pub type W = crate::W<CAP_TIMER_CFG_SPEC>;
#[doc = "Field `TIMER_EN` reader - When set, capture timer incrementing under APB_clk is enabled."]
pub type TIMER_EN_R = crate::BitReader;
#[doc = "Field `TIMER_EN` writer - When set, capture timer incrementing under APB_clk is enabled."]
pub type TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCI_EN` reader - When set, capture timer sync is enabled."]
pub type SYNCI_EN_R = crate::BitReader;
#[doc = "Field `SYNCI_EN` writer - When set, capture timer sync is enabled."]
pub type SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYNCI_SEL` reader - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
pub type SYNCI_SEL_R = crate::FieldReader;
#[doc = "Field `SYNCI_SEL` writer - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
pub type SYNCI_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `SYNC_SW` writer - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register."]
pub type SYNC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - When set, capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn timer_en(&self) -> TIMER_EN_R {
        TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - When set, capture timer sync is enabled."]
    #[inline(always)]
    pub fn synci_en(&self) -> SYNCI_EN_R {
        SYNCI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn synci_sel(&self) -> SYNCI_SEL_R {
        SYNCI_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_CFG")
            .field("timer_en", &self.timer_en())
            .field("synci_en", &self.synci_en())
            .field("synci_sel", &self.synci_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - When set, capture timer incrementing under APB_clk is enabled."]
    #[inline(always)]
    pub fn timer_en(&mut self) -> TIMER_EN_W<'_, CAP_TIMER_CFG_SPEC> {
        TIMER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - When set, capture timer sync is enabled."]
    #[inline(always)]
    pub fn synci_en(&mut self) -> SYNCI_EN_W<'_, CAP_TIMER_CFG_SPEC> {
        SYNCI_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix"]
    #[inline(always)]
    pub fn synci_sel(&mut self) -> SYNCI_SEL_W<'_, CAP_TIMER_CFG_SPEC> {
        SYNCI_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register."]
    #[inline(always)]
    pub fn sync_sw(&mut self) -> SYNC_SW_W<'_, CAP_TIMER_CFG_SPEC> {
        SYNC_SW_W::new(self, 5)
    }
}
#[doc = "Configure capture timer\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_TIMER_CFG_SPEC;
impl crate::RegisterSpec for CAP_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_cfg::R`](R) reader structure"]
impl crate::Readable for CAP_TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_cfg::W`](W) writer structure"]
impl crate::Writable for CAP_TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CAP_TIMER_CFG to value 0"]
impl crate::Resettable for CAP_TIMER_CFG_SPEC {}
