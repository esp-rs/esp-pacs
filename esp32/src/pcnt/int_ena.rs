#[doc = "Register `INT_ENA` reader"]
pub struct R(crate::R<INT_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<INT_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<INT_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<INT_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `INT_ENA` writer"]
pub struct W(crate::W<INT_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<INT_ENA_SPEC>;
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
impl From<crate::W<INT_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<INT_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CNT_THR_EVENT_U0` reader - This is the interrupt enable bit for channel0 event."]
pub type CNT_THR_EVENT_U0_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U0` writer - This is the interrupt enable bit for channel0 event."]
pub type CNT_THR_EVENT_U0_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U1` reader - This is the interrupt enable bit for channel1 event."]
pub type CNT_THR_EVENT_U1_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U1` writer - This is the interrupt enable bit for channel1 event."]
pub type CNT_THR_EVENT_U1_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U2` reader - This is the interrupt enable bit for channel2 event."]
pub type CNT_THR_EVENT_U2_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U2` writer - This is the interrupt enable bit for channel2 event."]
pub type CNT_THR_EVENT_U2_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U3` reader - This is the interrupt enable bit for channel3 event."]
pub type CNT_THR_EVENT_U3_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U3` writer - This is the interrupt enable bit for channel3 event."]
pub type CNT_THR_EVENT_U3_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U4` reader - This is the interrupt enable bit for channel4 event."]
pub type CNT_THR_EVENT_U4_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U4` writer - This is the interrupt enable bit for channel4 event."]
pub type CNT_THR_EVENT_U4_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U5` reader - This is the interrupt enable bit for channel5 event."]
pub type CNT_THR_EVENT_U5_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U5` writer - This is the interrupt enable bit for channel5 event."]
pub type CNT_THR_EVENT_U5_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U6` reader - This is the interrupt enable bit for channel6 event."]
pub type CNT_THR_EVENT_U6_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U6` writer - This is the interrupt enable bit for channel6 event."]
pub type CNT_THR_EVENT_U6_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
#[doc = "Field `CNT_THR_EVENT_U7` reader - This is the interrupt enable bit for channel7 event."]
pub type CNT_THR_EVENT_U7_R = crate::BitReader;
#[doc = "Field `CNT_THR_EVENT_U7` writer - This is the interrupt enable bit for channel7 event."]
pub type CNT_THR_EVENT_U7_W<'a, const O: u8> = crate::BitWriter<'a, INT_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0 - This is the interrupt enable bit for channel0 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u0(&self) -> CNT_THR_EVENT_U0_R {
        CNT_THR_EVENT_U0_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for channel1 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u1(&self) -> CNT_THR_EVENT_U1_R {
        CNT_THR_EVENT_U1_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for channel2 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u2(&self) -> CNT_THR_EVENT_U2_R {
        CNT_THR_EVENT_U2_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for channel3 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u3(&self) -> CNT_THR_EVENT_U3_R {
        CNT_THR_EVENT_U3_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for channel4 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u4(&self) -> CNT_THR_EVENT_U4_R {
        CNT_THR_EVENT_U4_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for channel5 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u5(&self) -> CNT_THR_EVENT_U5_R {
        CNT_THR_EVENT_U5_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for channel6 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u6(&self) -> CNT_THR_EVENT_U6_R {
        CNT_THR_EVENT_U6_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for channel7 event."]
    #[inline(always)]
    pub fn cnt_thr_event_u7(&self) -> CNT_THR_EVENT_U7_R {
        CNT_THR_EVENT_U7_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field(
                "cnt_thr_event_u0",
                &format_args!("{}", self.cnt_thr_event_u0().bit()),
            )
            .field(
                "cnt_thr_event_u1",
                &format_args!("{}", self.cnt_thr_event_u1().bit()),
            )
            .field(
                "cnt_thr_event_u2",
                &format_args!("{}", self.cnt_thr_event_u2().bit()),
            )
            .field(
                "cnt_thr_event_u3",
                &format_args!("{}", self.cnt_thr_event_u3().bit()),
            )
            .field(
                "cnt_thr_event_u4",
                &format_args!("{}", self.cnt_thr_event_u4().bit()),
            )
            .field(
                "cnt_thr_event_u5",
                &format_args!("{}", self.cnt_thr_event_u5().bit()),
            )
            .field(
                "cnt_thr_event_u6",
                &format_args!("{}", self.cnt_thr_event_u6().bit()),
            )
            .field(
                "cnt_thr_event_u7",
                &format_args!("{}", self.cnt_thr_event_u7().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - This is the interrupt enable bit for channel0 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u0(&mut self) -> CNT_THR_EVENT_U0_W<0> {
        CNT_THR_EVENT_U0_W::new(self)
    }
    #[doc = "Bit 1 - This is the interrupt enable bit for channel1 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u1(&mut self) -> CNT_THR_EVENT_U1_W<1> {
        CNT_THR_EVENT_U1_W::new(self)
    }
    #[doc = "Bit 2 - This is the interrupt enable bit for channel2 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u2(&mut self) -> CNT_THR_EVENT_U2_W<2> {
        CNT_THR_EVENT_U2_W::new(self)
    }
    #[doc = "Bit 3 - This is the interrupt enable bit for channel3 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u3(&mut self) -> CNT_THR_EVENT_U3_W<3> {
        CNT_THR_EVENT_U3_W::new(self)
    }
    #[doc = "Bit 4 - This is the interrupt enable bit for channel4 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u4(&mut self) -> CNT_THR_EVENT_U4_W<4> {
        CNT_THR_EVENT_U4_W::new(self)
    }
    #[doc = "Bit 5 - This is the interrupt enable bit for channel5 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u5(&mut self) -> CNT_THR_EVENT_U5_W<5> {
        CNT_THR_EVENT_U5_W::new(self)
    }
    #[doc = "Bit 6 - This is the interrupt enable bit for channel6 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u6(&mut self) -> CNT_THR_EVENT_U6_W<6> {
        CNT_THR_EVENT_U6_W::new(self)
    }
    #[doc = "Bit 7 - This is the interrupt enable bit for channel7 event."]
    #[inline(always)]
    #[must_use]
    pub fn cnt_thr_event_u7(&mut self) -> CNT_THR_EVENT_U7_W<7> {
        CNT_THR_EVENT_U7_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [int_ena](index.html) module"]
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [int_ena::R](R) reader structure"]
impl crate::Readable for INT_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [int_ena::W](W) writer structure"]
impl crate::Writable for INT_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets INT_ENA to value 0"]
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
