#[doc = "Register `SAR_CLK_DIV` reader"]
pub struct R(crate::R<SAR_CLK_DIV_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR_CLK_DIV_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR_CLK_DIV_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR_CLK_DIV_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR_CLK_DIV` writer"]
pub struct W(crate::W<SAR_CLK_DIV_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR_CLK_DIV_SPEC>;
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
impl From<crate::W<SAR_CLK_DIV_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR_CLK_DIV_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_CLK_DIV_NUM` reader - xxxx"]
pub type SAR2_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SAR2_CLK_DIV_NUM` writer - xxxx"]
pub type SAR2_CLK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_CLK_DIV_SPEC, 8, O>;
#[doc = "Field `SAR1_CLK_DIV_NUM` reader - xxxx"]
pub type SAR1_CLK_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SAR1_CLK_DIV_NUM` writer - xxxx"]
pub type SAR1_CLK_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, SAR_CLK_DIV_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    pub fn sar2_clk_div_num(&self) -> SAR2_CLK_DIV_NUM_R {
        SAR2_CLK_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - xxxx"]
    #[inline(always)]
    pub fn sar1_clk_div_num(&self) -> SAR1_CLK_DIV_NUM_R {
        SAR1_CLK_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR_CLK_DIV")
            .field(
                "sar2_clk_div_num",
                &format_args!("{}", self.sar2_clk_div_num().bits()),
            )
            .field(
                "sar1_clk_div_num",
                &format_args!("{}", self.sar1_clk_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR_CLK_DIV_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_clk_div_num(&mut self) -> SAR2_CLK_DIV_NUM_W<0> {
        SAR2_CLK_DIV_NUM_W::new(self)
    }
    #[doc = "Bits 8:15 - xxxx"]
    #[inline(always)]
    #[must_use]
    pub fn sar1_clk_div_num(&mut self) -> SAR1_CLK_DIV_NUM_W<8> {
        SAR1_CLK_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "xxxx\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar_clk_div](index.html) module"]
pub struct SAR_CLK_DIV_SPEC;
impl crate::RegisterSpec for SAR_CLK_DIV_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar_clk_div::R](R) reader structure"]
impl crate::Readable for SAR_CLK_DIV_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar_clk_div::W](W) writer structure"]
impl crate::Writable for SAR_CLK_DIV_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR_CLK_DIV to value 0x0404"]
impl crate::Resettable for SAR_CLK_DIV_SPEC {
    const RESET_VALUE: Self::Ux = 0x0404;
}
