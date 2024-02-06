#[doc = "Register `PRO_DCACHE_DBUG7` reader"]
pub type R = crate::R<PRO_DCACHE_DBUG7_SPEC>;
#[doc = "Field `PRO_IRAM1ADDR_IA` reader - "]
pub type PRO_IRAM1ADDR_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn pro_iram1addr_ia(&self) -> PRO_IRAM1ADDR_IA_R {
        PRO_IRAM1ADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG7")
            .field(
                "pro_iram1addr_ia",
                &format_args!("{}", self.pro_iram1addr_ia().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<PRO_DCACHE_DBUG7_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug7::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_DBUG7_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG7_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_dbug7::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG7_SPEC {}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG7 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG7_SPEC {
    const RESET_VALUE: u32 = 0;
}
