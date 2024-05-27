///Register `PRO_DCACHE_DBUG9` reader
pub type R = crate::R<PRO_DCACHE_DBUG9_SPEC>;
///Field `PRO_OPSDRAMADDR_IA` reader -
pub type PRO_OPSDRAMADDR_IA_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:19
    #[inline(always)]
    pub fn pro_opsdramaddr_ia(&self) -> PRO_OPSDRAMADDR_IA_R {
        PRO_OPSDRAMADDR_IA_R::new(self.bits & 0x000f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_DCACHE_DBUG9")
            .field("pro_opsdramaddr_ia", &self.pro_opsdramaddr_ia())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_dcache_dbug9::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_DCACHE_DBUG9_SPEC;
impl crate::RegisterSpec for PRO_DCACHE_DBUG9_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_dcache_dbug9::R`](R) reader structure
impl crate::Readable for PRO_DCACHE_DBUG9_SPEC {}
///`reset()` method sets PRO_DCACHE_DBUG9 to value 0
impl crate::Resettable for PRO_DCACHE_DBUG9_SPEC {
    const RESET_VALUE: u32 = 0;
}
