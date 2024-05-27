///Register `IDLE` reader
pub type R = crate::R<IDLE_SPEC>;
///Field `IDLE` reader - The content of this bit is 1 when the RSA accelerator is idle.
pub type IDLE_R = crate::BitReader;
impl R {
    ///Bit 0 - The content of this bit is 1 when the RSA accelerator is idle.
    #[inline(always)]
    pub fn idle(&self) -> IDLE_R {
        IDLE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDLE").field("idle", &self.idle()).finish()
    }
}
/**RSA idle register

You can [`read`](crate::generic::Reg::read) this register and get [`idle::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IDLE_SPEC;
impl crate::RegisterSpec for IDLE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`idle::R`](R) reader structure
impl crate::Readable for IDLE_SPEC {}
///`reset()` method sets IDLE to value 0
impl crate::Resettable for IDLE_SPEC {
    const RESET_VALUE: u32 = 0;
}
