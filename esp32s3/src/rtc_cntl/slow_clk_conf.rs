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
#[doc = "Field `ANA_CLK_DIV_VLD` reader - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
pub type ANA_CLK_DIV_VLD_R = crate::BitReader;
#[doc = "Field `ANA_CLK_DIV_VLD` writer - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
pub type ANA_CLK_DIV_VLD_W<'a, const O: u8> = crate::BitWriter<'a, SLOW_CLK_CONF_SPEC, O>;
#[doc = "Field `ANA_CLK_DIV` reader - rtc clk div"]
pub type ANA_CLK_DIV_R = crate::FieldReader;
#[doc = "Field `ANA_CLK_DIV` writer - rtc clk div"]
pub type ANA_CLK_DIV_W<'a, const O: u8> = crate::FieldWriter<'a, SLOW_CLK_CONF_SPEC, 8, O>;
#[doc = "Field `SLOW_CLK_NEXT_EDGE` reader - No public"]
pub type SLOW_CLK_NEXT_EDGE_R = crate::BitReader;
#[doc = "Field `SLOW_CLK_NEXT_EDGE` writer - No public"]
pub type SLOW_CLK_NEXT_EDGE_W<'a, const O: u8> = crate::BitWriter<'a, SLOW_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bit 22 - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
    #[inline(always)]
    pub fn ana_clk_div_vld(&self) -> ANA_CLK_DIV_VLD_R {
        ANA_CLK_DIV_VLD_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bits 23:30 - rtc clk div"]
    #[inline(always)]
    pub fn ana_clk_div(&self) -> ANA_CLK_DIV_R {
        ANA_CLK_DIV_R::new(((self.bits >> 23) & 0xff) as u8)
    }
    #[doc = "Bit 31 - No public"]
    #[inline(always)]
    pub fn slow_clk_next_edge(&self) -> SLOW_CLK_NEXT_EDGE_R {
        SLOW_CLK_NEXT_EDGE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SLOW_CLK_CONF")
            .field(
                "ana_clk_div_vld",
                &format_args!("{}", self.ana_clk_div_vld().bit()),
            )
            .field(
                "ana_clk_div",
                &format_args!("{}", self.ana_clk_div().bits()),
            )
            .field(
                "slow_clk_next_edge",
                &format_args!("{}", self.slow_clk_next_edge().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SLOW_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 22 - used to sync div bus. clear vld before set reg_rtc_ana_clk_div, then set vld to actually switch the clk"]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_div_vld(&mut self) -> ANA_CLK_DIV_VLD_W<22> {
        ANA_CLK_DIV_VLD_W::new(self)
    }
    #[doc = "Bits 23:30 - rtc clk div"]
    #[inline(always)]
    #[must_use]
    pub fn ana_clk_div(&mut self) -> ANA_CLK_DIV_W<23> {
        ANA_CLK_DIV_W::new(self)
    }
    #[doc = "Bit 31 - No public"]
    #[inline(always)]
    #[must_use]
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
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SLOW_CLK_CONF to value 0x0040_0000"]
impl crate::Resettable for SLOW_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0040_0000;
}
