///Register `TCBCNT` reader
pub type R = crate::R<TCBCNT_SPEC>;
///Field `TCBCNT` reader - Number of bytes transferred by CIU unit to card.
pub type TCBCNT_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - Number of bytes transferred by CIU unit to card.
    #[inline(always)]
    pub fn tcbcnt(&self) -> TCBCNT_R {
        TCBCNT_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TCBCNT")
            .field("tcbcnt", &self.tcbcnt())
            .finish()
    }
}
/**Transferred byte count register

You can [`read`](crate::generic::Reg::read) this register and get [`tcbcnt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TCBCNT_SPEC;
impl crate::RegisterSpec for TCBCNT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`tcbcnt::R`](R) reader structure
impl crate::Readable for TCBCNT_SPEC {}
///`reset()` method sets TCBCNT to value 0
impl crate::Resettable for TCBCNT_SPEC {
    const RESET_VALUE: u32 = 0;
}
