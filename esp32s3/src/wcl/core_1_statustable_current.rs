#[doc = "Register `Core_1_STATUSTABLE_CURRENT` reader"]
pub struct R(crate::R<CORE_1_STATUSTABLE_CURRENT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CORE_1_STATUSTABLE_CURRENT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CORE_1_STATUSTABLE_CURRENT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CORE_1_STATUSTABLE_CURRENT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `Core_1_STATUSTABLE_CURRENT` writer"]
pub struct W(crate::W<CORE_1_STATUSTABLE_CURRENT_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CORE_1_STATUSTABLE_CURRENT_SPEC>;
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
impl From<crate::W<CORE_1_STATUSTABLE_CURRENT_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CORE_1_STATUSTABLE_CURRENT_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `CORE_1_STATUSTABLE_CURRENT` reader - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
pub type CORE_1_STATUSTABLE_CURRENT_R = crate::FieldReader<u16>;
#[doc = "Field `CORE_1_STATUSTABLE_CURRENT` writer - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
pub type CORE_1_STATUSTABLE_CURRENT_W<'a, const O: u8> =
    crate::FieldWriter<'a, CORE_1_STATUSTABLE_CURRENT_SPEC, 13, O, u16>;
impl R {
    #[doc = "Bits 1:13 - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
    #[inline(always)]
    pub fn core_1_statustable_current(&self) -> CORE_1_STATUSTABLE_CURRENT_R {
        CORE_1_STATUSTABLE_CURRENT_R::new(((self.bits >> 1) & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("Core_1_STATUSTABLE_CURRENT")
            .field(
                "core_1_statustable_current",
                &format_args!("{}", self.core_1_statustable_current().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CORE_1_STATUSTABLE_CURRENT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bits 1:13 - This field is used to quickly read and rewrite the current field of all STATUSTABLE registers,for example,bit 1 represents the current field of STATUSTABLE1"]
    #[inline(always)]
    #[must_use]
    pub fn core_1_statustable_current(&mut self) -> CORE_1_STATUSTABLE_CURRENT_W<1> {
        CORE_1_STATUSTABLE_CURRENT_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Status register of statustable current\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [core_1_statustable_current](index.html) module"]
pub struct CORE_1_STATUSTABLE_CURRENT_SPEC;
impl crate::RegisterSpec for CORE_1_STATUSTABLE_CURRENT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [core_1_statustable_current::R](R) reader structure"]
impl crate::Readable for CORE_1_STATUSTABLE_CURRENT_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [core_1_statustable_current::W](W) writer structure"]
impl crate::Writable for CORE_1_STATUSTABLE_CURRENT_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets Core_1_STATUSTABLE_CURRENT to value 0"]
impl crate::Resettable for CORE_1_STATUSTABLE_CURRENT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
