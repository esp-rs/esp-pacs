#[doc = "Register `STATE1` reader"]
pub struct R(crate::R<STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `STATE1` reader - "]
pub type STATE1_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31"]
    #[inline(always)]
    pub fn state1(&self) -> STATE1_R {
        STATE1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATE1")
            .field("state1", &format_args!("{}", self.state1().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<STATE1_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [state1](index.html) module"]
pub struct STATE1_SPEC;
impl crate::RegisterSpec for STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [state1::R](R) reader structure"]
impl crate::Readable for STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets STATE1 to value 0"]
impl crate::Resettable for STATE1_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
