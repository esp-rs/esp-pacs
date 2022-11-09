#[doc = "Register `INT_CLR` writer"]
pub struct W(crate::W<INT_CLR_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_CLR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<INT_CLR_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_CLR_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_THR_EVENT_U0_INT_CLR` writer - Set this bit to clear channel0 event interrupt."]
pub type CNT_THR_EVENT_U0_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U1_INT_CLR` writer - Set this bit to clear channel1 event interrupt."]
pub type CNT_THR_EVENT_U1_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U2_INT_CLR` writer - Set this bit to clear channel2 event interrupt."]
pub type CNT_THR_EVENT_U2_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U3_INT_CLR` writer - Set this bit to clear channel3 event interrupt."]
pub type CNT_THR_EVENT_U3_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U4_INT_CLR` writer - Set this bit to clear channel4 event interrupt."]
pub type CNT_THR_EVENT_U4_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U5_INT_CLR` writer - Set this bit to clear channel5 event interrupt."]
pub type CNT_THR_EVENT_U5_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U6_INT_CLR` writer - Set this bit to clear channel6 event interrupt."]
pub type CNT_THR_EVENT_U6_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
#[doc = "Field `CNT_THR_EVENT_U7_INT_CLR` writer - Set this bit to clear channel7 event interrupt."]
pub type CNT_THR_EVENT_U7_INT_CLR_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, INT_CLR_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Set this bit to clear channel0 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u0_int_clr(&mut self) -> CNT_THR_EVENT_U0_INT_CLR_W<0> {
        CNT_THR_EVENT_U0_INT_CLR_W::new(self)
    }
    #[doc = "Bit 1 - Set this bit to clear channel1 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u1_int_clr(&mut self) -> CNT_THR_EVENT_U1_INT_CLR_W<1> {
        CNT_THR_EVENT_U1_INT_CLR_W::new(self)
    }
    #[doc = "Bit 2 - Set this bit to clear channel2 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u2_int_clr(&mut self) -> CNT_THR_EVENT_U2_INT_CLR_W<2> {
        CNT_THR_EVENT_U2_INT_CLR_W::new(self)
    }
    #[doc = "Bit 3 - Set this bit to clear channel3 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u3_int_clr(&mut self) -> CNT_THR_EVENT_U3_INT_CLR_W<3> {
        CNT_THR_EVENT_U3_INT_CLR_W::new(self)
    }
    #[doc = "Bit 4 - Set this bit to clear channel4 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u4_int_clr(&mut self) -> CNT_THR_EVENT_U4_INT_CLR_W<4> {
        CNT_THR_EVENT_U4_INT_CLR_W::new(self)
    }
    #[doc = "Bit 5 - Set this bit to clear channel5 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u5_int_clr(&mut self) -> CNT_THR_EVENT_U5_INT_CLR_W<5> {
        CNT_THR_EVENT_U5_INT_CLR_W::new(self)
    }
    #[doc = "Bit 6 - Set this bit to clear channel6 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u6_int_clr(&mut self) -> CNT_THR_EVENT_U6_INT_CLR_W<6> {
        CNT_THR_EVENT_U6_INT_CLR_W::new(self)
    }
    #[doc = "Bit 7 - Set this bit to clear channel7 event interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u7_int_clr(&mut self) -> CNT_THR_EVENT_U7_INT_CLR_W<7> {
        CNT_THR_EVENT_U7_INT_CLR_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_clr](index.html) module"]
pub struct INT_CLR_SPEC;
impl crate::RegisterSpec for INT_CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [int_clr::W](W) writer structure"]
impl crate::Writable for INT_CLR_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_CLR to value 0"]
impl crate::Resettable for INT_CLR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
