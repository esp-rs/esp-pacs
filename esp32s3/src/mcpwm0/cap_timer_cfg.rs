///Register `CAP_TIMER_CFG` reader
pub type R = crate::R<CAP_TIMER_CFG_SPEC>;
///Register `CAP_TIMER_CFG` writer
pub type W = crate::W<CAP_TIMER_CFG_SPEC>;
///Field `CAP_TIMER_EN` reader - When set, capture timer incrementing under APB_clk is enabled.
pub type CAP_TIMER_EN_R = crate::BitReader;
///Field `CAP_TIMER_EN` writer - When set, capture timer incrementing under APB_clk is enabled.
pub type CAP_TIMER_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAP_SYNCI_EN` reader - When set, capture timer sync is enabled.
pub type CAP_SYNCI_EN_R = crate::BitReader;
///Field `CAP_SYNCI_EN` writer - When set, capture timer sync is enabled.
pub type CAP_SYNCI_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `CAP_SYNCI_SEL` reader - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix
pub type CAP_SYNCI_SEL_R = crate::FieldReader;
///Field `CAP_SYNCI_SEL` writer - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix
pub type CAP_SYNCI_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `CAP_SYNC_SW` writer - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register.
pub type CAP_SYNC_SW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - When set, capture timer incrementing under APB_clk is enabled.
    #[inline(always)]
    pub fn cap_timer_en(&self) -> CAP_TIMER_EN_R {
        CAP_TIMER_EN_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - When set, capture timer sync is enabled.
    #[inline(always)]
    pub fn cap_synci_en(&self) -> CAP_SYNCI_EN_R {
        CAP_SYNCI_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix
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
    ///Bit 0 - When set, capture timer incrementing under APB_clk is enabled.
    #[inline(always)]
    #[must_use]
    pub fn cap_timer_en(&mut self) -> CAP_TIMER_EN_W<CAP_TIMER_CFG_SPEC> {
        CAP_TIMER_EN_W::new(self, 0)
    }
    ///Bit 1 - When set, capture timer sync is enabled.
    #[inline(always)]
    #[must_use]
    pub fn cap_synci_en(&mut self) -> CAP_SYNCI_EN_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNCI_EN_W::new(self, 1)
    }
    ///Bits 2:4 - capture module sync input selection. 0: none, 1: timer0 sync_out, 2: timer1 sync_out, 3: timer2 sync_out, 4: SYNC0 from GPIO matrix, 5: SYNC1 from GPIO matrix, 6: SYNC2 from GPIO matrix
    #[inline(always)]
    #[must_use]
    pub fn cap_synci_sel(&mut self) -> CAP_SYNCI_SEL_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNCI_SEL_W::new(self, 2)
    }
    ///Bit 5 - Write 1 will force a capture timer sync, capture timer is loaded with value in phase register.
    #[inline(always)]
    #[must_use]
    pub fn cap_sync_sw(&mut self) -> CAP_SYNC_SW_W<CAP_TIMER_CFG_SPEC> {
        CAP_SYNC_SW_W::new(self, 5)
    }
}
/**Configure capture timer

You can [`read`](crate::generic::Reg::read) this register and get [`cap_timer_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`cap_timer_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CAP_TIMER_CFG_SPEC;
impl crate::RegisterSpec for CAP_TIMER_CFG_SPEC {
    type Ux = u32;
}
///`read()` method returns [`cap_timer_cfg::R`](R) reader structure
impl crate::Readable for CAP_TIMER_CFG_SPEC {}
///`write(|w| ..)` method takes [`cap_timer_cfg::W`](W) writer structure
impl crate::Writable for CAP_TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets CAP_TIMER_CFG to value 0
impl crate::Resettable for CAP_TIMER_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
