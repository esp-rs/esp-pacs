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
#[doc = "Field `ANA_CLK_DIV_VLD` reader - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
pub type ANA_CLK_DIV_VLD_R = crate::BitReader<bool>;
#[doc = "Field `ANA_CLK_DIV_VLD` writer - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
pub type ANA_CLK_DIV_VLD_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLOW_CLK_CONF_SPEC, bool, O>;
#[doc = "Field `ANA_CLK_DIV` reader - Set the rtc_clk divider."]
pub type ANA_CLK_DIV_R = crate::FieldReader<u8, u8>;
#[doc = "Field `ANA_CLK_DIV` writer - Set the rtc_clk divider."]
pub type ANA_CLK_DIV_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, SLOW_CLK_CONF_SPEC, u8, u8, 8, O>;
#[doc = "Field `SLOW_CLK_NEXT_EDGE` reader - "]
pub type SLOW_CLK_NEXT_EDGE_R = crate::BitReader<bool>;
#[doc = "Field `SLOW_CLK_NEXT_EDGE` writer - "]
pub type SLOW_CLK_NEXT_EDGE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, SLOW_CLK_CONF_SPEC, bool, O>;
impl R {
    #[doc = "Bit 22 - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ana_clk_div_vld(&self) -> ANA_CLK_DIV_VLD_R {
        ANA_CLK_DIV_VLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - Set the rtc_clk divider."]
    #[inline(always)]
    pub fn ana_clk_div(&self) -> ANA_CLK_DIV_R {
        ANA_CLK_DIV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slow_clk_next_edge(&self) -> SLOW_CLK_NEXT_EDGE_R {
        SLOW_CLK_NEXT_EDGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
impl W {
    #[doc = "Bit 22 - Synchronizes the reg_rtc_ana_clk_div bus. Note that you have to invalidate the bus before switching clock, and validate the new clock."]
    #[inline(always)]
    pub fn ana_clk_div_vld(&mut self) -> ANA_CLK_DIV_VLD_W<22> {
        ANA_CLK_DIV_VLD_W::new(self)
    }
    #[doc = "Bits 23:30 - Set the rtc_clk divider."]
    #[inline(always)]
    pub fn ana_clk_div(&mut self) -> ANA_CLK_DIV_W<23> {
        ANA_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn slow_clk_next_edge(&mut self) -> SLOW_CLK_NEXT_EDGE_W<31> {
        SLOW_CLK_NEXT_EDGE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "RTC slow clock configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [slow_clk_conf](index.html) module"]
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
