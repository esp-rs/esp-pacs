#[doc = "Register `RD_KEY0_DATA4` reader"]
pub type R = crate::R<RD_KEY0_DATA4_SPEC>;
#[doc = "Field `KEY0_DATA4` reader - Stores the fourth 32 bits of KEY0."]
pub type KEY0_DATA4_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - Stores the fourth 32 bits of KEY0."]
    #[inline(always)]
    pub fn key0_data4(&self) -> KEY0_DATA4_R {
        KEY0_DATA4_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_KEY0_DATA4")
            .field("key0_data4", &self.key0_data4())
            .finish()
    }
}
#[doc = "Register $n of BLOCK4 (KEY0).\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`rd_key0_data4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_KEY0_DATA4_SPEC;
impl crate::RegisterSpec for RD_KEY0_DATA4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_key0_data4::R`](R) reader structure"]
impl crate::Readable for RD_KEY0_DATA4_SPEC {}
#[doc = "`reset()` method sets RD_KEY0_DATA4 to value 0"]
impl crate::Resettable for RD_KEY0_DATA4_SPEC {
    const RESET_VALUE: u32 = 0;
}
