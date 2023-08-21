#[doc = "Register `REF_CNT_RST` writer"]
pub type W = crate::W<REF_CNT_RST_SPEC>;
#[doc = "Field `CH[TX_REF_CNT_RST_CH0,TX_REF_CNT_RST_CH1,TX_REF_CNT_RST_CH2,TX_REF_CNT_RST_CH3,RX_REF_CNT_RST_CH4,RX_REF_CNT_RST_CH5,RX_REF_CNT_RST_CH6,RX_REF_CNT_RST_CH7]` writer - This register is used to reset the clock divider of CHANNEL0."]
pub type CH_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
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
    pub unsafe fn ch<const O: u8>(&mut self) -> CH_W<REF_CNT_RST_SPEC, O> {
        CH_W::new(self)
    }
    #[doc = "Bit 0 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch0(&mut self) -> CH_W<REF_CNT_RST_SPEC, 0> {
        CH_W::new(self)
    }
    #[doc = "Bit 1 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch1(&mut self) -> CH_W<REF_CNT_RST_SPEC, 1> {
        CH_W::new(self)
    }
    #[doc = "Bit 2 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch2(&mut self) -> CH_W<REF_CNT_RST_SPEC, 2> {
        CH_W::new(self)
    }
    #[doc = "Bit 3 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chtx_ref_cnt_rst_ch3(&mut self) -> CH_W<REF_CNT_RST_SPEC, 3> {
        CH_W::new(self)
    }
    #[doc = "Bit 4 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch4(&mut self) -> CH_W<REF_CNT_RST_SPEC, 4> {
        CH_W::new(self)
    }
    #[doc = "Bit 5 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch5(&mut self) -> CH_W<REF_CNT_RST_SPEC, 5> {
        CH_W::new(self)
    }
    #[doc = "Bit 6 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch6(&mut self) -> CH_W<REF_CNT_RST_SPEC, 6> {
        CH_W::new(self)
    }
    #[doc = "Bit 7 - This register is used to reset the clock divider of CHANNEL0."]
    #[inline(always)]
    #[must_use]
    pub fn chrx_ref_cnt_rst_ch7(&mut self) -> CH_W<REF_CNT_RST_SPEC, 7> {
        CH_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "RMT clock divider reset register\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ref_cnt_rst::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REF_CNT_RST_SPEC;
impl crate::RegisterSpec for REF_CNT_RST_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`ref_cnt_rst::W`](W) writer structure"]
impl crate::Writable for REF_CNT_RST_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets REF_CNT_RST to value 0"]
impl crate::Resettable for REF_CNT_RST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
