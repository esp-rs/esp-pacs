#[doc = "Register `CIS_CONF_W0` reader"]
pub struct R(crate::R<CIS_CONF_W0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CIS_CONF_W0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CIS_CONF_W0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CIS_CONF_W0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `CIS_CONF_W0` writer"]
pub struct W(crate::W<CIS_CONF_W0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CIS_CONF_W0_SPEC>;
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
impl From<crate::W<CIS_CONF_W0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CIS_CONF_W0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CIS_CONF_W0` reader - Configure cis addr 39~36"]
pub type CIS_CONF_W0_R = crate::FieldReader<u32>;
#[doc = "Field `CIS_CONF_W0` writer - Configure cis addr 39~36"]
pub type CIS_CONF_W0_W<'a, const O: u8> = crate::FieldWriter<'a, CIS_CONF_W0_SPEC, 32, O, u32>;
impl R {
    #[doc = "Bits 0:31 - Configure cis addr 39~36"]
    #[inline(always)]
    pub fn cis_conf_w0(&self) -> CIS_CONF_W0_R {
        CIS_CONF_W0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CIS_CONF_W0")
            .field(
                "cis_conf_w0",
                &format_args!("{}", self.cis_conf_w0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CIS_CONF_W0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31 - Configure cis addr 39~36"]
    #[inline(always)]
    #[must_use]
    pub fn cis_conf_w0(&mut self) -> CIS_CONF_W0_W<0> {
        CIS_CONF_W0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SDIO cis configuration register\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [cis_conf_w0](index.html) module"]
pub struct CIS_CONF_W0_SPEC;
impl crate::RegisterSpec for CIS_CONF_W0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [cis_conf_w0::R](R) reader structure"]
impl crate::Readable for CIS_CONF_W0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [cis_conf_w0::W](W) writer structure"]
impl crate::Writable for CIS_CONF_W0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CIS_CONF_W0 to value 0xffff_ffff"]
impl crate::Resettable for CIS_CONF_W0_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
