#[doc = "Register `PERI_RST_EN` reader"]
pub struct R(crate::R<PERI_RST_EN_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PERI_RST_EN_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PERI_RST_EN_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PERI_RST_EN_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PERI_RST_EN` writer"]
pub struct W(crate::W<PERI_RST_EN_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PERI_RST_EN_SPEC>;
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
impl From<crate::W<PERI_RST_EN_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PERI_RST_EN_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PERI_RST_EN` reader - "]
pub type PERI_RST_EN_R = crate::FieldReader<u32, u32>;
#[doc = "Field `PERI_RST_EN` writer - "]
pub type PERI_RST_EN_W<'a, const O: u8> = crate::FieldWriter<'a, PERI_RST_EN_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn peri_rst_en(&self) -> PERI_RST_EN_R {
        PERI_RST_EN_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PERI_RST_EN")
            .field(
                "peri_rst_en",
                &format_args!("{}", self.peri_rst_en().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PERI_RST_EN_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn peri_rst_en(&mut self) -> PERI_RST_EN_W<0> {
        PERI_RST_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [peri_rst_en](index.html) module"]
pub struct PERI_RST_EN_SPEC;
impl crate::RegisterSpec for PERI_RST_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [peri_rst_en::R](R) reader structure"]
impl crate::Readable for PERI_RST_EN_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [peri_rst_en::W](W) writer structure"]
impl crate::Writable for PERI_RST_EN_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PERI_RST_EN to value 0"]
impl crate::Resettable for PERI_RST_EN_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
