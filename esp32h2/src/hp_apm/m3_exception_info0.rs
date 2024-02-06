#[doc = "Register `M3_EXCEPTION_INFO0` reader"]
pub type R = crate::R<M3_EXCEPTION_INFO0_SPEC>;
#[doc = "Field `M3_EXCEPTION_REGION` reader - Exception region"]
pub type M3_EXCEPTION_REGION_R = crate::FieldReader<u16>;
#[doc = "Field `M3_EXCEPTION_MODE` reader - Exception mode"]
pub type M3_EXCEPTION_MODE_R = crate::FieldReader;
#[doc = "Field `M3_EXCEPTION_ID` reader - Exception id information"]
pub type M3_EXCEPTION_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Exception region"]
    #[inline(always)]
    pub fn m3_exception_region(&self) -> M3_EXCEPTION_REGION_R {
        M3_EXCEPTION_REGION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Exception mode"]
    #[inline(always)]
    pub fn m3_exception_mode(&self) -> M3_EXCEPTION_MODE_R {
        M3_EXCEPTION_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:22 - Exception id information"]
    #[inline(always)]
    pub fn m3_exception_id(&self) -> M3_EXCEPTION_ID_R {
        M3_EXCEPTION_ID_R::new(((self.bits >> 18) & 0x1f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M3_EXCEPTION_INFO0")
            .field(
                "m3_exception_region",
                &format_args!("{}", self.m3_exception_region().bits()),
            )
            .field(
                "m3_exception_mode",
                &format_args!("{}", self.m3_exception_mode().bits()),
            )
            .field(
                "m3_exception_id",
                &format_args!("{}", self.m3_exception_id().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<M3_EXCEPTION_INFO0_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "M3 exception_info0 register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`m3_exception_info0::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M3_EXCEPTION_INFO0_SPEC;
impl crate::RegisterSpec for M3_EXCEPTION_INFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m3_exception_info0::R`](R) reader structure"]
impl crate::Readable for M3_EXCEPTION_INFO0_SPEC {}
#[doc = "`reset()` method sets M3_EXCEPTION_INFO0 to value 0"]
impl crate::Resettable for M3_EXCEPTION_INFO0_SPEC {
    const RESET_VALUE: u32 = 0;
}
