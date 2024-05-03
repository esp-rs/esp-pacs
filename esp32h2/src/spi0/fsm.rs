#[doc = "Register `FSM` reader"]
pub type R = crate::R<FSM_SPEC>;
#[doc = "Register `FSM` writer"]
pub type W = crate::W<FSM_SPEC>;
#[doc = "Field `LOCK_DELAY_TIME` reader - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type LOCK_DELAY_TIME_R = crate::FieldReader;
#[doc = "Field `LOCK_DELAY_TIME` writer - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
pub type LOCK_DELAY_TIME_W<'a, REG> = crate::FieldWriter<'a, REG, 5>;
impl R {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    pub fn lock_delay_time(&self) -> LOCK_DELAY_TIME_R {
        LOCK_DELAY_TIME_R::new(((self.bits >> 7) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM")
            .field("lock_delay_time", &self.lock_delay_time().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<FSM_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 7:11 - The lock delay time of SPI0/1 arbiter by spi0_slv_st, after PER is sent by SPI1."]
    #[inline(always)]
    #[must_use]
    pub fn lock_delay_time(&mut self) -> LOCK_DELAY_TIME_W<FSM_SPEC> {
        LOCK_DELAY_TIME_W::new(self, 7)
    }
}
#[doc = "SPI0 FSM status register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`fsm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`fsm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FSM_SPEC;
impl crate::RegisterSpec for FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fsm::R`](R) reader structure"]
impl crate::Readable for FSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fsm::W`](W) writer structure"]
impl crate::Writable for FSM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets FSM to value 0x0200"]
impl crate::Resettable for FSM_SPEC {
    const RESET_VALUE: u32 = 0x0200;
}
