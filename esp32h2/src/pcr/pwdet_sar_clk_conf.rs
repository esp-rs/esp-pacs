#[doc = "Register `PWDET_SAR_CLK_CONF` reader"]
pub struct R(crate::R<PWDET_SAR_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PWDET_SAR_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PWDET_SAR_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PWDET_SAR_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PWDET_SAR_CLK_CONF` writer"]
pub struct W(crate::W<PWDET_SAR_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PWDET_SAR_CLK_CONF_SPEC>;
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
impl From<crate::W<PWDET_SAR_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PWDET_SAR_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PWDET_SAR_CLK_DIV_NUM` reader - xxxx"]
pub type PWDET_SAR_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `PWDET_SAR_CLK_DIV_NUM` writer - xxxx"]
pub type PWDET_SAR_CLK_DIV_NUM_W<'a, const O: u8> =
    crate::FieldWriter<'a, PWDET_SAR_CLK_CONF_SPEC, 8, O>;
#[doc = "Field `PWDET_SAR_READER_EN` reader - xxxx"]
pub type PWDET_SAR_READER_EN_R = crate::BitReader;
#[doc = "Field `PWDET_SAR_READER_EN` writer - xxxx"]
pub type PWDET_SAR_READER_EN_W<'a, const O: u8> = crate::BitWriter<'a, PWDET_SAR_CLK_CONF_SPEC, O>;
impl R {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    pub fn pwdet_sar_clk_div_num(&self) -> PWDET_SAR_CLK_DIV_NUM_R {
        PWDET_SAR_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bit 8 - xxxx"]
    #[inline(always)]
    pub fn pwdet_sar_reader_en(&self) -> PWDET_SAR_READER_EN_R {
        PWDET_SAR_READER_EN_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PWDET_SAR_CLK_CONF")
            .field(
                "pwdet_sar_clk_div_num",
                &format_args!("{}", self.pwdet_sar_clk_div_num().bits()),
            )
            .field(
                "pwdet_sar_reader_en",
                &format_args!("{}", self.pwdet_sar_reader_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PWDET_SAR_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_sar_clk_div_num(&mut self) -> PWDET_SAR_CLK_DIV_NUM_W<0> {
        PWDET_SAR_CLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bit 8 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn pwdet_sar_reader_en(&mut self) -> PWDET_SAR_READER_EN_W<8> {
        PWDET_SAR_READER_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xxxx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [pwdet_sar_clk_conf](index.html) module"]
pub struct PWDET_SAR_CLK_CONF_SPEC;
impl crate::RegisterSpec for PWDET_SAR_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [pwdet_sar_clk_conf::R](R) reader structure"]
impl crate::Readable for PWDET_SAR_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [pwdet_sar_clk_conf::W](W) writer structure"]
impl crate::Writable for PWDET_SAR_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PWDET_SAR_CLK_CONF to value 0x0107"]
impl crate::Resettable for PWDET_SAR_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x0107;
}
