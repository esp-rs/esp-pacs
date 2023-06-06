#[doc = "Register `STRAP` reader"]
pub struct R(crate::R<STRAP_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STRAP_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STRAP_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STRAP_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STRAPPING` reader - GPIO strapping values: bit4 ~ bit2 correspond to stripping pins GPIO45, GPIO0, and GPIO46 respectively."]
pub type STRAPPING_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - GPIO strapping values: bit4 ~ bit2 correspond to stripping pins GPIO45, GPIO0, and GPIO46 respectively."]
    #[inline(always)]
    pub fn strapping(&self) -> STRAPPING_R {
        STRAPPING_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STRAP")
            .field("strapping", &format_args!("{}", self.strapping().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STRAP_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Bootstrap pin value register\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [strap](index.html) module"]
pub struct STRAP_SPEC;
impl crate::RegisterSpec for STRAP_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [strap::R](R) reader structure"]
impl crate::Readable for STRAP_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STRAP to value 0"]
impl crate::Resettable for STRAP_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
