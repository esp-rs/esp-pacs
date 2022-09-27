#[doc = "Register `REF_CNT_RST` writer"]
pub struct W(crate::W<REF_CNT_RST_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<REF_CNT_RST_SPEC>;
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
impl From<crate::W<REF_CNT_RST_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<REF_CNT_RST_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CH[TX_CH0,TX_CH1,TX_CH2,TX_CH3,RX_CH0,RX_CH1,RX_CH2,RX_CH3]` writer - This register is used to reset the clock divider of CHANNEL0."]
pub type CH_W<'a, const O: u8> = crate::BitWriter<'a, u32, REF_CNT_RST_SPEC, bool, O>;
impl W {
    #[doc = "This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub unsafe fn ch<const O: u8>(&mut self) -> CH_W<O> {
        CH_W::new(self)
    }
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch0(&mut self) -> CH_W<0> {
        CH_W::new(self)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch1(&mut self) -> CH_W<1> {
        CH_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch2(&mut self) -> CH_W<2> {
        CH_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chtx_ch3(&mut self) -> CH_W<3> {
        CH_W::new(self)
    }
    #[doc = "Bit 4 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch0(&mut self) -> CH_W<4> {
        CH_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch1(&mut self) -> CH_W<5> {
        CH_W::new(self)
    }
    #[doc = "Bit 6 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch2(&mut self) -> CH_W<6> {
        CH_W::new(self)
    }
    #[doc = "Bit 7 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    pub fn chrx_ch3(&mut self) -> CH_W<7> {
        CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RMT clock divider reset register\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ref_cnt_rst](index.html) module"]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [ref_cnt_rst::W](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0
    }
}
