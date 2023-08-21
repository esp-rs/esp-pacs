#[doc = "Register `T0_%s` reader"]
pub type R = crate::R<T0__SPEC>;
#[doc = "Field `T0` reader - This register stores the %sth 32-bit piece of 128-bit T0"]
pub type T0_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - This register stores the %sth 32-bit piece of 128-bit T0"]
    #[inline(always)]
    pub fn t0(&self) -> T0_R {
        T0_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("T0_")
            .field("t0", &format_args!("{}", self.t0().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<T0__SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "T0\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`t0_::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct T0__SPEC;
impl crate::RegisterSpec for T0__SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`t0_::R`](R) reader structure"]
impl crate::Readable for T0__SPEC {}
#[doc = "`reset()` method sets T0_%s to value 0"]
impl crate::Resettable for T0__SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
