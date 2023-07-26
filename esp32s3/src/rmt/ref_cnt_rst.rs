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
#[doc = "Field `CH[TX_REF_CNT_RST_CH0,TX_REF_CNT_RST_CH1,TX_REF_CNT_RST_CH2,TX_REF_CNT_RST_CH3,RX_REF_CNT_RST_CH4,RX_REF_CNT_RST_CH5,RX_REF_CNT_RST_CH6,RX_REF_CNT_RST_CH7]` writer - This register is used to reset the clock divider of CHANNEL0."]
pub type CH_W<'a, const O: u8> = crate::BitWriter<'a, REF_CNT_RST_SPEC, O>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<REF_CNT_RST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub unsafe fn ch<const O: u8>(&mut self) -> CH_W<O> {
        CH_W::new(self)
    }
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch0(&mut self) -> CH_W<0> {
        CH_W::new(self)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch1(&mut self) -> CH_W<1> {
        CH_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch2(&mut self) -> CH_W<2> {
        CH_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch3(&mut self) -> CH_W<3> {
        CH_W::new(self)
    }
    #[doc = "Bit 4 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch4(&mut self) -> CH_W<4> {
        CH_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch5(&mut self) -> CH_W<5> {
        CH_W::new(self)
    }
    #[doc = "Bit 6 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch6(&mut self) -> CH_W<6> {
        CH_W::new(self)
    }
    #[doc = "Bit 7 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch7(&mut self) -> CH_W<7> {
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
