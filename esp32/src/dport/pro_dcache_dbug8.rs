#[doc = "Register `PRO_DCACHE_DBUG8` reader"]
pub type R = crate::R<PRO_DCACHE_DBUG8_SPEC>;
#[doc = "Field `PRO_IROM0ADDR_IA` reader - "]
pub type PRO_IROM0ADDR_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn pro_irom0addr_ia(&self) -> PRO_IROM0ADDR_IA_R {
        PRO_IROM0ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG8")
            .field(
                "pro_irom0addr_ia",
                &format_args!("{}", self.pro_irom0addr_ia().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_DBUG8_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug8::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_DBUG8_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG8_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_dbug8::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG8_SPEC {}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG8 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG8_SPEC {
    const RESET_VALUE: u32 = 0;
}
