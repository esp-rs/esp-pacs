#[doc = "Register `M5_EXCEPTION_INFO0` reader"]
pub type R = crate::R<M5_EXCEPTION_INFO0_SPEC>;
#[doc = "Field `M5_EXCEPTION_REGION` reader - Represents exception region."]
pub type M5_EXCEPTION_REGION_R = crate::FieldReader;
#[doc = "Field `M5_EXCEPTION_MODE` reader - Represents exception mode."]
pub type M5_EXCEPTION_MODE_R = crate::FieldReader;
#[doc = "Field `M5_EXCEPTION_ID` reader - Represents exception id information."]
pub type M5_EXCEPTION_ID_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:7 - Represents exception region."]
    #[inline(always)]
    pub fn m5_exception_region(&self) -> M5_EXCEPTION_REGION_R {
        M5_EXCEPTION_REGION_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 16:17 - Represents exception mode."]
    #[inline(always)]
    pub fn m5_exception_mode(&self) -> M5_EXCEPTION_MODE_R {
        M5_EXCEPTION_MODE_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bits 18:23 - Represents exception id information."]
    #[inline(always)]
    pub fn m5_exception_id(&self) -> M5_EXCEPTION_ID_R {
        M5_EXCEPTION_ID_R::new(((self.bits >> 18) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("M5_EXCEPTION_INFO0")
            .field("m5_exception_region", &self.m5_exception_region())
            .field("m5_exception_mode", &self.m5_exception_mode())
            .field("m5_exception_id", &self.m5_exception_id())
            .finish()
    }
}
#[doc = "M5 exception_info0 register\n\nYou can [`read`](crate::Reg::read) this register and get [`m5_exception_info0::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct M5_EXCEPTION_INFO0_SPEC;
impl crate::RegisterSpec for M5_EXCEPTION_INFO0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`m5_exception_info0::R`](R) reader structure"]
impl crate::Readable for M5_EXCEPTION_INFO0_SPEC {}
#[doc = "`reset()` method sets M5_EXCEPTION_INFO0 to value 0"]
impl crate::Resettable for M5_EXCEPTION_INFO0_SPEC {}
