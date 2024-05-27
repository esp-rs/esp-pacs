///Register `DONE` reader
pub type R = crate::R<DONE_SPEC>;
///Field `FLASH_DONE` reader - Set this bit when encryption operation is complete.
pub type FLASH_DONE_R = crate::BitReader;
impl R {
    ///Bit 0 - Set this bit when encryption operation is complete.
    #[inline(always)]
    pub fn flash_done(&self) -> FLASH_DONE_R {
        FLASH_DONE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DONE")
            .field("flash_done", &self.flash_done())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`done::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DONE_SPEC;
impl crate::RegisterSpec for DONE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`done::R`](R) reader structure
impl crate::Readable for DONE_SPEC {}
///`reset()` method sets DONE to value 0
impl crate::Resettable for DONE_SPEC {
    const RESET_VALUE: u32 = 0;
}
