#[doc = "Register `INT_ENA` reader"]
pub type R = crate::R<INT_ENA_SPEC>;
#[doc = "Register `INT_ENA` writer"]
pub type W = crate::W<INT_ENA_SPEC>;
#[doc = "Field `CNT_THR_EVENT_U(0-7)` reader - This is the interrupt enable bit for channel%s event."]
pub type CNT_THR_EVENT_U_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U(0-7)` writer - This is the interrupt enable bit for channel%s event."]
pub type CNT_THR_EVENT_U_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "This is the interrupt enable bit for channel(0-7) event."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field"]
    #[inline(always)]
    pub fn cnt_thr_event_u(&self, n: u8) -> CNT_THR_EVENT_U_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNT_THR_EVENT_U_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "This is the interrupt enable bit for channel(0-7) event."]
    #[inline(always)]
    pub fn cnt_thr_event_u_iter(&self) -> impl Iterator<Item = CNT_THR_EVENT_U_R> + '_ {
        (0..8).map(move |n| CNT_THR_EVENT_U_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - This is the interrupt enable bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7(&self) -> CNT_THR_EVENT_U_R {
        CNT_THR_EVENT_U_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("cnt_thr_event_u0", &self.cnt_thr_event_u0())
            .field("cnt_thr_event_u1", &self.cnt_thr_event_u1())
            .field("cnt_thr_event_u2", &self.cnt_thr_event_u2())
            .field("cnt_thr_event_u3", &self.cnt_thr_event_u3())
            .field("cnt_thr_event_u4", &self.cnt_thr_event_u4())
            .field("cnt_thr_event_u5", &self.cnt_thr_event_u5())
            .field("cnt_thr_event_u6", &self.cnt_thr_event_u6())
            .field("cnt_thr_event_u7", &self.cnt_thr_event_u7())
            .finish()
    }
}
impl W {
    #[doc = "This is the interrupt enable bit for channel(0-7) event."]
    #[doc = ""]
    #[doc = "NOTE: `n` is number of field in register. `n == 0` corresponds to `CNT_THR_EVENT_U0` field"]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u(&mut self, n: u8) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        CNT_THR_EVENT_U_W::new(self, n)
    }
    #[doc = "Bit 0 - This is the interrupt enable bit for channel0 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u0(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for channel1 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u1(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 1)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for channel2 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u2(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 2)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for channel3 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u3(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 3)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for channel4 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u4(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 4)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for channel5 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u5(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 5)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for channel6 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u6(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 6)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for channel7 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u7(&mut self) -> CNT_THR_EVENT_U_W<INT_ENA_SPEC> {
        CNT_THR_EVENT_U_W::new(self, 7)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_ena::R`](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_ena::W`](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
