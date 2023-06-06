#[doc = "Register `WDTCONFIG1` reader"]
pub struct R(crate::R<WDTCONFIG1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<WDTCONFIG1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<WDTCONFIG1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<WDTCONFIG1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `WDTCONFIG1` writer"]
pub struct W(crate::W<WDTCONFIG1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<WDTCONFIG1_SPEC>;
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
impl From<crate::W<WDTCONFIG1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<WDTCONFIG1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `WDT_DIVCNT_RST` writer - reg_wdt_divcnt_rst."]
pub type WDT_DIVCNT_RST_W<'a, const O: u8> = crate::BitWriter<'a, WDTCONFIG1_SPEC, O>;
#[doc = "Field `WDT_CLK_PRESCALE` reader - reg_wdt_clk_prescale."]
pub type WDT_CLK_PRESCALE_R = crate::FieldReader<u16>;
#[doc = "Field `WDT_CLK_PRESCALE` writer - reg_wdt_clk_prescale."]
pub type WDT_CLK_PRESCALE_W<'a, const O: u8> = crate::FieldWriter<'a, WDTCONFIG1_SPEC, 16, O, u16>;
impl R {
    #[doc = "Bits 16:31 - reg_wdt_clk_prescale."]
    #[inline(always)]
    pub fn wdt_clk_prescale(&self) -> WDT_CLK_PRESCALE_R {
        WDT_CLK_PRESCALE_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WDTCONFIG1")
            .field(
                "wdt_clk_prescale",
                &format_args!("{}", self.wdt_clk_prescale().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<WDTCONFIG1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - reg_wdt_divcnt_rst."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_divcnt_rst(&mut self) -> WDT_DIVCNT_RST_W<0> {
        WDT_DIVCNT_RST_W::new(self)
    }
    #[doc = "Bits 16:31 - reg_wdt_clk_prescale."]
    #[inline(always)]
    #[must_use]
    pub fn wdt_clk_prescale(&mut self) -> WDT_CLK_PRESCALE_W<16> {
        WDT_CLK_PRESCALE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "TIMG_WDTCONFIG1_REG.\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [wdtconfig1](index.html) module"]
pub struct WDTCONFIG1_SPEC;
impl crate::RegisterSpec for WDTCONFIG1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [wdtconfig1::R](R) reader structure"]
impl crate::Readable for WDTCONFIG1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [wdtconfig1::W](W) writer structure"]
impl crate::Writable for WDTCONFIG1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets WDTCONFIG1 to value 0x0001_0000"]
impl crate::Resettable for WDTCONFIG1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0001_0000;
}
