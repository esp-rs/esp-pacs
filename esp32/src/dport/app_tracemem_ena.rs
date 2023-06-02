#[doc = "Register `APP_TRACEMEM_ENA` reader"]
pub struct R(crate::R<APP_TRACEMEM_ENA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_TRACEMEM_ENA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_TRACEMEM_ENA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_TRACEMEM_ENA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_TRACEMEM_ENA` writer"]
pub struct W(crate::W<APP_TRACEMEM_ENA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_TRACEMEM_ENA_SPEC>;
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
impl From<crate::W<APP_TRACEMEM_ENA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_TRACEMEM_ENA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APP_TRACEMEM_ENA` reader - "]
pub type APP_TRACEMEM_ENA_R = crate::BitReader;
#[doc = "Field `APP_TRACEMEM_ENA` writer - "]
pub type APP_TRACEMEM_ENA_W<'a, const O: u8> = crate::BitWriter<'a, APP_TRACEMEM_ENA_SPEC, O>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn app_tracemem_ena(&self) -> APP_TRACEMEM_ENA_R {
        APP_TRACEMEM_ENA_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_TRACEMEM_ENA")
            .field(
                "app_tracemem_ena",
                &format_args!("{}", self.app_tracemem_ena().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_TRACEMEM_ENA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn app_tracemem_ena(&mut self) -> APP_TRACEMEM_ENA_W<0> {
        APP_TRACEMEM_ENA_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_tracemem_ena](index.html) module"]
pub struct APP_TRACEMEM_ENA_SPEC;
impl crate::RegisterSpec for APP_TRACEMEM_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_tracemem_ena::R](R) reader structure"]
impl crate::Readable for APP_TRACEMEM_ENA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_tracemem_ena::W](W) writer structure"]
impl crate::Writable for APP_TRACEMEM_ENA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_TRACEMEM_ENA to value 0"]
impl crate::Resettable for APP_TRACEMEM_ENA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
