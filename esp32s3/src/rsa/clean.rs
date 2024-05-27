///Register `CLEAN` reader
pub type R = crate::R<CLEAN_SPEC>;
///Field `CLEAN` reader - The content of this bit is 1 when memories complete initialization.
pub type CLEAN_R = crate::BitReader;
impl R {
    ///Bit 0 - The content of this bit is 1 when memories complete initialization.
    #[inline(always)]
    pub fn clean(&self) -> CLEAN_R {
        CLEAN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLEAN")
            .field("clean", &self.clean())
            .finish()
    }
}
/**RSA clean register

You can [`read`](crate::generic::Reg::read) this register and get [`clean::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct CLEAN_SPEC;
impl crate::RegisterSpec for CLEAN_SPEC {
    type Ux = u32;
}
///`read()` method returns [`clean::R`](R) reader structure
impl crate::Readable for CLEAN_SPEC {}
///`reset()` method sets CLEAN to value 0
impl crate::Resettable for CLEAN_SPEC {
    const RESET_VALUE: u32 = 0;
}
