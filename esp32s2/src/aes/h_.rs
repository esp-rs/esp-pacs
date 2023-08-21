#[doc = "Register `H_%s` reader"]
pub type R = crate::R<H__SPEC>;
#[doc = "Field `H` reader - GCM hash subkey"]
pub type H_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - GCM hash subkey"]
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("H_")
            .field("h", &format_args!("{}", self.h().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<H__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "GCM hash subkey\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`h_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct H__SPEC;
impl crate::RegisterSpec for H__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`h_::R`](R) reader structure"]
impl crate::Readable for H__SPEC {}
#[doc = "`reset()` method sets H_%s to value 0"]
impl crate::Resettable for H__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
