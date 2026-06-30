#[doc = "Register `M6_EXCEPTION_INFO0` reader"]
pub type R = crate::R<M6_EXCEPTION_INFO0_SPEC>;
#[doc = "Field `M6_EXCEPTION_REGION` reader - Represents exception region."]
pub type M6_EXCEPTION_REGION_R = crate::FieldReader<u16>;
#[doc = "Field `M6_EXCEPTION_MODE` reader - Represents exception mode."]
pub type M6_EXCEPTION_MODE_R = crate::FieldReader;
#[doc = "Field `M6_EXCEPTION_ID` reader - Represents exception id information."]
pub type M6_EXCEPTION_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:15 - Represents exception region."]
    #[inline(always)]
    pub fn m6_exception_region(&self) -> M6_EXCEPTION_REGION_R {
        M6_EXCEPTION_REGION_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Represents exception mode."]
    #[inline(always)]
    pub fn m6_exception_mode(&self) -> M6_EXCEPTION_MODE_R {
        M6_EXCEPTION_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:24 - Represents exception id information."]
    #[inline(always)]
    pub fn m6_exception_id(&self) -> M6_EXCEPTION_ID_R {
        M6_EXCEPTION_ID_R::new(((self.bits >> 18) & 0x7f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M6_EXCEPTION_INFO0")
            .field("m6_exception_region", &self.m6_exception_region())
            .field("m6_exception_mode", &self.m6_exception_mode())
            .field("m6_exception_id", &self.m6_exception_id())
            .finish()
    }
}
#[doc = "M6 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m6_exception_info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M6_EXCEPTION_INFO0_SPEC;
impl crate::RegisterSpec for M6_EXCEPTION_INFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m6_exception_info0::R`](R) reader structure"]
impl crate::Readable for M6_EXCEPTION_INFO0_SPEC {}
#[doc = "`reset()` method sets M6_EXCEPTION_INFO0 to value 0"]
impl crate::Resettable for M6_EXCEPTION_INFO0_SPEC {}
