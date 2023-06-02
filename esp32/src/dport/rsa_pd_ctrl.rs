#[doc = "Register `RSA_PD_CTRL` reader"]
pub struct R(crate::R<RSA_PD_CTRL_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<RSA_PD_CTRL_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<RSA_PD_CTRL_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<RSA_PD_CTRL_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `RSA_PD_CTRL` writer"]
pub struct W(crate::W<RSA_PD_CTRL_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<RSA_PD_CTRL_SPEC>;
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
impl From<crate::W<RSA_PD_CTRL_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<RSA_PD_CTRL_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `RSA_PD` reader - "]
pub type RSA_PD_R = crate::BitReader;
#[doc = "Field `RSA_PD` writer - "]
pub type RSA_PD_W<'a, const O: u8> = crate::BitWriter<'a, RSA_PD_CTRL_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn rsa_pd(&self) -> RSA_PD_R {
        RSA_PD_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RSA_PD_CTRL")
            .field("rsa_pd", &format_args!("{}", self.rsa_pd().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RSA_PD_CTRL_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn rsa_pd(&mut self) -> RSA_PD_W<0> {
        RSA_PD_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [rsa_pd_ctrl](index.html) module"]
pub struct RSA_PD_CTRL_SPEC;
impl crate::RegisterSpec for RSA_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [rsa_pd_ctrl::R](R) reader structure"]
impl crate::Readable for RSA_PD_CTRL_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [rsa_pd_ctrl::W](W) writer structure"]
impl crate::Writable for RSA_PD_CTRL_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets RSA_PD_CTRL to value 0"]
impl crate::Resettable for RSA_PD_CTRL_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
