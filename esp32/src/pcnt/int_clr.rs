#[doc = "Register `INT_CLR` writer"]
pub type W = crate::W<INT_CLR_SPEC>;
#[doc = "Field `CNT_THR_EVENT_U0` writer - Set this bit to clear channel0 event interrupt."]
pub type CNT_THR_EVENT_U0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U1` writer - Set this bit to clear channel1 event interrupt."]
pub type CNT_THR_EVENT_U1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U2` writer - Set this bit to clear channel2 event interrupt."]
pub type CNT_THR_EVENT_U2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U3` writer - Set this bit to clear channel3 event interrupt."]
pub type CNT_THR_EVENT_U3_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U4` writer - Set this bit to clear channel4 event interrupt."]
pub type CNT_THR_EVENT_U4_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U5` writer - Set this bit to clear channel5 event interrupt."]
pub type CNT_THR_EVENT_U5_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U6` writer - Set this bit to clear channel6 event interrupt."]
pub type CNT_THR_EVENT_U6_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CNT_THR_EVENT_U7` writer - Set this bit to clear channel7 event interrupt."]
pub type CNT_THR_EVENT_U7_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear channel0 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u0(&mut self) -> CNT_THR_EVENT_U0_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U0_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear channel1 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u1(&mut self) -> CNT_THR_EVENT_U1_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U1_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear channel2 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u2(&mut self) -> CNT_THR_EVENT_U2_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U2_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear channel3 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u3(&mut self) -> CNT_THR_EVENT_U3_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U3_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear channel4 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u4(&mut self) -> CNT_THR_EVENT_U4_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U4_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear channel5 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u5(&mut self) -> CNT_THR_EVENT_U5_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U5_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear channel6 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u6(&mut self) -> CNT_THR_EVENT_U6_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U6_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear channel7 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u7(&mut self) -> CNT_THR_EVENT_U7_W<INT_CLR_SPEC> {
        CNT_THR_EVENT_U7_W::new(self, 7)
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
#[doc = "\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`int_clr::W`](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
