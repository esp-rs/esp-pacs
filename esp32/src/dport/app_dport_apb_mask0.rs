#[doc = "Register `APP_DPORT_APB_MASK0` reader"]
pub struct R(crate::R<APP_DPORT_APB_MASK0_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<APP_DPORT_APB_MASK0_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<APP_DPORT_APB_MASK0_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<APP_DPORT_APB_MASK0_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `APP_DPORT_APB_MASK0` writer"]
pub struct W(crate::W<APP_DPORT_APB_MASK0_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<APP_DPORT_APB_MASK0_SPEC>;
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
impl From<crate::W<APP_DPORT_APB_MASK0_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<APP_DPORT_APB_MASK0_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `APPDPORT_APB_MASK0` reader - "]
pub type APPDPORT_APB_MASK0_R = crate::FieldReader<u32, u32>;
#[doc = "Field `APPDPORT_APB_MASK0` writer - "]
pub type APPDPORT_APB_MASK0_W<'a, const O: u8> =
    crate::FieldWriter<'a, APP_DPORT_APB_MASK0_SPEC, 32, O, u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn appdport_apb_mask0(&self) -> APPDPORT_APB_MASK0_R {
        APPDPORT_APB_MASK0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APP_DPORT_APB_MASK0")
            .field(
                "appdport_apb_mask0",
                &format_args!("{}", self.appdport_apb_mask0().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<APP_DPORT_APB_MASK0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    #[must_use]
    pub fn appdport_apb_mask0(&mut self) -> APPDPORT_APB_MASK0_W<0> {
        APPDPORT_APB_MASK0_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [app_dport_apb_mask0](index.html) module"]
pub struct APP_DPORT_APB_MASK0_SPEC;
impl crate::RegisterSpec for APP_DPORT_APB_MASK0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [app_dport_apb_mask0::R](R) reader structure"]
impl crate::Readable for APP_DPORT_APB_MASK0_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [app_dport_apb_mask0::W](W) writer structure"]
impl crate::Writable for APP_DPORT_APB_MASK0_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets APP_DPORT_APB_MASK0 to value 0"]
impl crate::Resettable for APP_DPORT_APB_MASK0_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
