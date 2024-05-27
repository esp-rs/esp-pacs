///Register `H_MEM[%s]` reader
pub type R = crate::R<H_MEM_SPEC>;
///Field `H` reader - GCM hash subkey
pub type H_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - GCM hash subkey
    #[inline(always)]
    pub fn h(&self) -> H_R {
        H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("H_MEM").field("h", &self.h()).finish()
    }
}
/**GCM hash subkey

You can [`read`](crate::generic::Reg::read) this register and get [`h_mem::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct H_MEM_SPEC;
impl crate::RegisterSpec for H_MEM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`h_mem::R`](R) reader structure
impl crate::Readable for H_MEM_SPEC {}
///`reset()` method sets H_MEM[%s] to value 0
impl crate::Resettable for H_MEM_SPEC {
    const RESET_VALUE: u32 = 0;
}
