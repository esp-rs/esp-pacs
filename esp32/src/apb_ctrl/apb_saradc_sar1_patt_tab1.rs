#[doc = "Register `APB_SARADC_SAR1_PATT_TAB1` reader"]
pub struct R(crate::R<APB_SARADC_SAR1_PATT_TAB1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APB_SARADC_SAR1_PATT_TAB1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APB_SARADC_SAR1_PATT_TAB1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APB_SARADC_SAR1_PATT_TAB1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APB_SARADC_SAR1_PATT_TAB1` writer"]
pub struct W(crate::W<APB_SARADC_SAR1_PATT_TAB1_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APB_SARADC_SAR1_PATT_TAB1_SPEC>;
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
impl From<crate::W<APB_SARADC_SAR1_PATT_TAB1_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APB_SARADC_SAR1_PATT_TAB1_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SARADC_SAR1_PATT_TAB1` reader - item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub type SARADC_SAR1_PATT_TAB1_R = crate::FieldReader<u32, u32>;
#[doc = "Field `SARADC_SAR1_PATT_TAB1` writer - item 0 ~ 3 for pattern table 1 (each item one byte)"]
pub type SARADC_SAR1_PATT_TAB1_W<'a, const O: u8> =
    crate::FieldWriter<'a, APB_SARADC_SAR1_PATT_TAB1_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    pub fn saradc_sar1_patt_tab1(&self) -> SARADC_SAR1_PATT_TAB1_R {
        SARADC_SAR1_PATT_TAB1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_SAR1_PATT_TAB1")
            .field(
                "saradc_sar1_patt_tab1",
                &format_args!("{}", self.saradc_sar1_patt_tab1().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APB_SARADC_SAR1_PATT_TAB1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - item 0 ~ 3 for pattern table 1 (each item one byte)"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sar1_patt_tab1(&mut self) -> SARADC_SAR1_PATT_TAB1_W<0> {
        SARADC_SAR1_PATT_TAB1_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [apb_saradc_sar1_patt_tab1](index.html) module"]
pub struct APB_SARADC_SAR1_PATT_TAB1_SPEC;
impl crate::RegisterSpec for APB_SARADC_SAR1_PATT_TAB1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [apb_saradc_sar1_patt_tab1::R](R) reader structure"]
impl crate::Readable for APB_SARADC_SAR1_PATT_TAB1_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [apb_saradc_sar1_patt_tab1::W](W) writer structure"]
impl crate::Writable for APB_SARADC_SAR1_PATT_TAB1_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APB_SARADC_SAR1_PATT_TAB1 to value 0x0f0f_0f0f"]
impl crate::Resettable for APB_SARADC_SAR1_PATT_TAB1_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
