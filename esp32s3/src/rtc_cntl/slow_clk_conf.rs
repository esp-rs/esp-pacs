#[doc = "Register `SLOW_CLK_CONF` reader"]
pub struct R(crate::R<SLOW_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SLOW_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SLOW_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SLOW_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SLOW_CLK_CONF` writer"]
pub struct W(crate::W<SLOW_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SLOW_CLK_CONF_SPEC>;
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
impl From<crate::W<SLOW_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SLOW_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RTC_ANA_CLK_DIV_VLD` reader - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
pub type RTC_ANA_CLK_DIV_VLD_R = crate::BitReader<bool>;
#[doc = "Field `RTC_ANA_CLK_DIV_VLD` writer - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
pub type RTC_ANA_CLK_DIV_VLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLOW_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `RTC_ANA_CLK_DIV` reader - rtc clk div"]
pub type RTC_ANA_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `RTC_ANA_CLK_DIV` writer - rtc clk div"]
pub type RTC_ANA_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLOW_CLK_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `RTC_SLOW_CLK_NEXT_EDGE` reader - No public"]
pub type RTC_SLOW_CLK_NEXT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `RTC_SLOW_CLK_NEXT_EDGE` writer - No public"]
pub type RTC_SLOW_CLK_NEXT_EDGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLOW_CLK_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 22 - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
    #[inline(always)]
    pub fn rtc_ana_clk_div_vld(&self) -> RTC_ANA_CLK_DIV_VLD_R {
        RTC_ANA_CLK_DIV_VLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - rtc clk div"]
    #[inline(always)]
    pub fn rtc_ana_clk_div(&self) -> RTC_ANA_CLK_DIV_R {
        RTC_ANA_CLK_DIV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - No public"]
    #[inline(always)]
    pub fn rtc_slow_clk_next_edge(&self) -> RTC_SLOW_CLK_NEXT_EDGE_R {
        RTC_SLOW_CLK_NEXT_EDGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
    #[inline(always)]
    pub fn rtc_ana_clk_div_vld(&mut self) -> RTC_ANA_CLK_DIV_VLD_W<22> {
        RTC_ANA_CLK_DIV_VLD_W::new(self)
    }
    #[doc = "Bits 23:30 - rtc clk div"]
    #[inline(always)]
    pub fn rtc_ana_clk_div(&mut self) -> RTC_ANA_CLK_DIV_W<23> {
        RTC_ANA_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 31 - No public"]
    #[inline(always)]
    pub fn rtc_slow_clk_next_edge(&mut self) -> RTC_SLOW_CLK_NEXT_EDGE_W<31> {
        RTC_SLOW_CLK_NEXT_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "configure slow clk\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_clk_conf](index.html) module"]
pub struct SLOW_CLK_CONF_SPEC;
impl crate::RegisterSpec for SLOW_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [slow_clk_conf::R](R) reader structure"]
impl crate::Readable for SLOW_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [slow_clk_conf::W](W) writer structure"]
impl crate::Writable for SLOW_CLK_CONF_SPEC {
    type Writer = W;
}
#[doc = "`reset()` method sets SLOW_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for SLOW_CLK_CONF_SPEC {
    #[inline(always)]
    fn reset_value() -> Self::Ux {
        0x0040_0000
    }
}
