#[doc = "Register `PRIVILEGE_MODE_SEL_LOCK` reader"]
pub struct R(crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<PRIVILEGE_MODE_SEL_LOCK_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `PRIVILEGE_MODE_SEL_LOCK` writer"]
pub struct W(crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>;
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
impl From<crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<PRIVILEGE_MODE_SEL_LOCK_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `PRIVILEGE_MODE_SEL_LOCK` reader - privilege_mode_sel_lock"]
pub type PRIVILEGE_MODE_SEL_LOCK_R = crate::BitReader;
#[doc = "Field `PRIVILEGE_MODE_SEL_LOCK` writer - privilege_mode_sel_lock"]
pub type PRIVILEGE_MODE_SEL_LOCK_W<'a, const O: u8> =
    crate::BitWriter<'a, PRIVILEGE_MODE_SEL_LOCK_SPEC, O>;
impl R {
    #[doc = "Bit 0 - privilege_mode_sel_lock"]
    #[inline(always)]
    pub fn privilege_mode_sel_lock(&self) -> PRIVILEGE_MODE_SEL_LOCK_R {
        PRIVILEGE_MODE_SEL_LOCK_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRIVILEGE_MODE_SEL_LOCK")
            .field(
                "privilege_mode_sel_lock",
                &format_args!("{}", self.privilege_mode_sel_lock().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRIVILEGE_MODE_SEL_LOCK_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - privilege_mode_sel_lock"]
    #[inline(always)]
    #[must_use]
    pub fn privilege_mode_sel_lock(&mut self) -> PRIVILEGE_MODE_SEL_LOCK_W<0> {
        PRIVILEGE_MODE_SEL_LOCK_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "SENSITIVE_PRIVILEGE_MODE_SEL_LOCK_REG\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [privilege_mode_sel_lock](index.html) module"]
pub struct PRIVILEGE_MODE_SEL_LOCK_SPEC;
impl crate::RegisterSpec for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [privilege_mode_sel_lock::R](R) reader structure"]
impl crate::Readable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [privilege_mode_sel_lock::W](W) writer structure"]
impl crate::Writable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets PRIVILEGE_MODE_SEL_LOCK to value 0"]
impl crate::Resettable for PRIVILEGE_MODE_SEL_LOCK_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
