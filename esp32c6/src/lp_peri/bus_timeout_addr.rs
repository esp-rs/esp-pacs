#[doc = "Register `BUS_TIMEOUT_ADDR` reader"]
pub struct R(crate::R<BUS_TIMEOUT_ADDR_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<BUS_TIMEOUT_ADDR_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<BUS_TIMEOUT_ADDR_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<BUS_TIMEOUT_ADDR_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `LP_PERI_TIMEOUT_ADDR` reader - need_des"]
pub type LP_PERI_TIMEOUT_ADDR_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn lp_peri_timeout_addr(&self) -> LP_PERI_TIMEOUT_ADDR_R {
        LP_PERI_TIMEOUT_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BUS_TIMEOUT_ADDR")
            .field(
                "lp_peri_timeout_addr",
                &format_args!("{}", self.lp_peri_timeout_addr().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BUS_TIMEOUT_ADDR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [bus_timeout_addr](index.html) module"]
pub struct BUS_TIMEOUT_ADDR_SPEC;
impl crate::RegisterSpec for BUS_TIMEOUT_ADDR_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [bus_timeout_addr::R](R) reader structure"]
impl crate::Readable for BUS_TIMEOUT_ADDR_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets BUS_TIMEOUT_ADDR to value 0"]
impl crate::Resettable for BUS_TIMEOUT_ADDR_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
