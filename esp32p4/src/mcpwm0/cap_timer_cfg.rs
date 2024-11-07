#[doc = "Register `CAP_TIMER_CFG` reader"]
pub type R = crate::R<CAP_TIMER_CFG_SPEC>;
#[doc = "Register `CAP_TIMER_CFG` writer"]
pub type W = crate::W<CAP_TIMER_CFG_SPEC>;
#[doc = "Field `CAP_TIMER_EN` reader - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
pub type CAP_TIMER_EN_R = crate::BitReader;
#[doc = "Field `CAP_TIMER_EN` writer - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
pub type CAP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SYNCI_EN` reader - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
pub type CAP_SYNCI_EN_R = crate::BitReader;
#[doc = "Field `CAP_SYNCI_EN` writer - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
pub type CAP_SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CAP_SYNCI_SEL` reader - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
pub type CAP_SYNCI_SEL_R = crate::FieldReader;
#[doc = "Field `CAP_SYNCI_SEL` writer - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
pub type CAP_SYNCI_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `CAP_SYNC_SW` writer - Configures the generation of a capture timer sync when reg_cap_synci_en is 1.\\\\0: Invalid, No effect\\\\1: Trigger a capture timer sync, capture timer is loaded with value in phase register"]
pub type CAP_SYNC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_timer_en(&self) -> CAP_TIMER_EN_R {
        CAP_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_synci_en(&self) -> CAP_SYNCI_EN_R {
        CAP_SYNCI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:4 - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
    #[inline(always)]
    pub fn cap_synci_sel(&self) -> CAP_SYNCI_SEL_R {
        CAP_SYNCI_SEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CAP_TIMER_CFG")
            .field("cap_timer_en", &self.cap_timer_en())
            .field("cap_synci_en", &self.cap_synci_en())
            .field("cap_synci_sel", &self.cap_synci_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures whether or not to enable capture timer increment.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_timer_en(&mut self) -> CAP_TIMER_EN_W<CAP_TIMER_CFG_SPEC> {
        CAP_TIMER_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures whether or not to enable capture timer sync.\\\\0: Disable\\\\1: Enable"]
    #[inline(always)]
    pub fn cap_synci_en(&mut self) -> CAP_SYNCI_EN_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNCI_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:4 - Configures the selection of capture module sync input.\\\\0: None\\\\1: Timer0 sync_out\\\\2: Timer1 sync_out\\\\3: Timer2 sync_out\\\\4: SYNC0 from GPIO matrix\\\\5: SYNC1 from GPIO matrix\\\\6: SYNC2 from GPIO matrix\\\\7: None"]
    #[inline(always)]
    pub fn cap_synci_sel(&mut self) -> CAP_SYNCI_SEL_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNCI_SEL_W::new(self, 2)
    }
    #[doc = "Bit 5 - Configures the generation of a capture timer sync when reg_cap_synci_en is 1.\\\\0: Invalid, No effect\\\\1: Trigger a capture timer sync, capture timer is loaded with value in phase register"]
    #[inline(always)]
    pub fn cap_sync_sw(&mut self) -> CAP_SYNC_SW_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNC_SW_W::new(self, 5)
    }
}
#[doc = "Capture timer configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`cap_timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`cap_timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CAP_TIMER_CFG_SPEC;
impl crate::RegisterSpec for CAP_TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`cap_timer_cfg::R`](R) reader structure"]
impl crate::Readable for CAP_TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`cap_timer_cfg::W`](W) writer structure"]
impl crate::Writable for CAP_TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CAP_TIMER_CFG to value 0"]
impl crate::Resettable for CAP_TIMER_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
