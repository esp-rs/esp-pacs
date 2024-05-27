#[doc = "Register `VERSION` reader"]
pub type R = crate::R<VERSION_SPEC>;
#[doc = "Field `VERSION` reader - NA"]
pub type VERSION_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - NA"]
    #[inline(always)]
    pub fn version(&self) -> VERSION_R {
        VERSION_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("VERSION")
            .field("version", &self.version())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`version::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct VERSION_SPEC;
impl crate::RegisterSpec for VERSION_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`version::R`](R) reader structure"]
impl crate::Readable for VERSION_SPEC {}
#[doc = "`reset()` method sets VERSION to value 0x3135_302a"]
impl crate::Resettable for VERSION_SPEC {
    const RESET_VALUE: u32 = 0x3135_302a;
}
