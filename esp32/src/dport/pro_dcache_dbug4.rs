#[doc = "Register `PRO_DCACHE_DBUG4` reader"]
pub type R = crate::R<PRO_DCACHE_DBUG4_SPEC>;
#[doc = "Field `PRO_DRAM1ADDR0_IA` reader - "]
pub type PRO_DRAM1ADDR0_IA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn pro_dram1addr0_ia(&self) -> PRO_DRAM1ADDR0_IA_R {
        PRO_DRAM1ADDR0_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG4")
            .field("pro_dram1addr0_ia", &self.pro_dram1addr0_ia())
            .finish()
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRO_DCACHE_DBUG4_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pro_dcache_dbug4::R`](R) reader structure"]
impl crate::Readable for PRO_DCACHE_DBUG4_SPEC {}
#[doc = "`reset()` method sets PRO_DCACHE_DBUG4 to value 0"]
impl crate::Resettable for PRO_DCACHE_DBUG4_SPEC {
    const RESET_VALUE: u32 = 0;
}
