#[doc = "Register `SAR2_PATT_TAB2` reader"]
pub struct R(crate::R<SAR2_PATT_TAB2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<SAR2_PATT_TAB2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<SAR2_PATT_TAB2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<SAR2_PATT_TAB2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `SAR2_PATT_TAB2` writer"]
pub struct W(crate::W<SAR2_PATT_TAB2_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<SAR2_PATT_TAB2_SPEC>;
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
impl From<crate::W<SAR2_PATT_TAB2_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<SAR2_PATT_TAB2_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `SAR2_PATT_TAB2` reader - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
pub type SAR2_PATT_TAB2_R = crate::FieldReader<u32>;
#[doc = "Field `SAR2_PATT_TAB2` writer - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
pub type SAR2_PATT_TAB2_W<'a, const O: u8> =
    crate::FieldWriter<'a, SAR2_PATT_TAB2_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    pub fn sar2_patt_tab2(&self) -> SAR2_PATT_TAB2_R {
        SAR2_PATT_TAB2_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_PATT_TAB2")
            .field(
                "sar2_patt_tab2",
                &format_args!("{}", self.sar2_patt_tab2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<SAR2_PATT_TAB2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Item 4 ~ 7 for pattern table 2 (each item one byte)"]
    #[inline(always)]
    #[must_use]
    pub fn sar2_patt_tab2(&mut self) -> SAR2_PATT_TAB2_W<0> {
        SAR2_PATT_TAB2_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Item 4 ~ 7 for pattern table 2 (each item one byte)\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [sar2_patt_tab2](index.html) module"]
pub struct SAR2_PATT_TAB2_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [sar2_patt_tab2::R](R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB2_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [sar2_patt_tab2::W](W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB2_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB2 to value 0x0f0f_0f0f"]
impl crate::Resettable for SAR2_PATT_TAB2_SPEC {
    const RESET_VALUE: Self::Ux = 0x0f0f_0f0f;
}
