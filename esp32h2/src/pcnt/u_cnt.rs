#[doc = "Register `U%s_CNT` reader"]
pub struct R(crate::R<U_CNT_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<U_CNT_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<U_CNT_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<U_CNT_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CNT` reader - This register stores the current pulse count value for unit %s."]
pub type CNT_R = crate::FieldReader<u16, u16>;
impl R {
    #[doc = "Bits 0:15 - This register stores the current pulse count value for unit %s."]
    #[inline(always)]
    pub fn cnt(&self) -> CNT_R {
        CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U_CNT")
            .field("cnt", &format_args!("{}", self.cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<U_CNT_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Counter value for unit %s\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [u_cnt](index.html) module"]
pub struct U_CNT_SPEC;
impl crate::RegisterSpec for U_CNT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [u_cnt::R](R) reader structure"]
impl crate::Readable for U_CNT_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets U%s_CNT to value 0"]
impl crate::Resettable for U_CNT_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
