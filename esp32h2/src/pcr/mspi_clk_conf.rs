#[doc = "Register `MSPI_CLK_CONF` reader"]
pub struct R(crate::R<MSPI_CLK_CONF_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MSPI_CLK_CONF_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MSPI_CLK_CONF_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MSPI_CLK_CONF_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `MSPI_CLK_CONF` writer"]
pub struct W(crate::W<MSPI_CLK_CONF_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<MSPI_CLK_CONF_SPEC>;
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
impl From<crate::W<MSPI_CLK_CONF_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<MSPI_CLK_CONF_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `MSPI_FAST_DIV_NUM` reader - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
pub type MSPI_FAST_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `MSPI_FAST_DIV_NUM` writer - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
pub type MSPI_FAST_DIV_NUM_W<'a, const O: u8> = crate::FieldWriter<'a, MSPI_CLK_CONF_SPEC, 8, O>;
impl R {
    #[doc = "Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
    #[inline(always)]
    pub fn mspi_fast_div_num(&self) -> MSPI_FAST_DIV_NUM_R {
        MSPI_FAST_DIV_NUM_R::new((self.bits & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MSPI_CLK_CONF")
            .field(
                "mspi_fast_div_num",
                &format_args!("{}", self.mspi_fast_div_num().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MSPI_CLK_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:7 - Set as one within (0,1,2) to generate div1(default)/div2/div4 of low-speed clock-source to drive clk_mspi_fast. Only avaiable whe the clck-source is a low-speed clock-source such as XTAL/FOSC."]
    #[inline(always)]
    #[must_use]
    pub fn mspi_fast_div_num(&mut self) -> MSPI_FAST_DIV_NUM_W<0> {
        MSPI_FAST_DIV_NUM_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "MSPI_CLK configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mspi_clk_conf](index.html) module"]
pub struct MSPI_CLK_CONF_SPEC;
impl crate::RegisterSpec for MSPI_CLK_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mspi_clk_conf::R](R) reader structure"]
impl crate::Readable for MSPI_CLK_CONF_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [mspi_clk_conf::W](W) writer structure"]
impl crate::Writable for MSPI_CLK_CONF_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MSPI_CLK_CONF to value 0"]
impl crate::Resettable for MSPI_CLK_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
