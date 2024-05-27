///Register `EXTMEM_REJECT_INT_ST` reader
pub type R = crate::R<EXTMEM_REJECT_INT_ST_SPEC>;
///Field `EXTMEM_REJECT_INT_ST` reader - The raw interrupt status bit for the EXTMEM_REJECT_INT interrupt.
pub type EXTMEM_REJECT_INT_ST_R = crate::BitReader;
impl R {
    ///Bit 0 - The raw interrupt status bit for the EXTMEM_REJECT_INT interrupt.
    #[inline(always)]
    pub fn extmem_reject_int_st(&self) -> EXTMEM_REJECT_INT_ST_R {
        EXTMEM_REJECT_INT_ST_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXTMEM_REJECT_INT_ST")
            .field("extmem_reject_int_st", &self.extmem_reject_int_st())
            .finish()
    }
}
/**Masked interrupt status of external RAM permission

You can [`read`](crate::generic::Reg::read) this register and get [`extmem_reject_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXTMEM_REJECT_INT_ST_SPEC;
impl crate::RegisterSpec for EXTMEM_REJECT_INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`extmem_reject_int_st::R`](R) reader structure
impl crate::Readable for EXTMEM_REJECT_INT_ST_SPEC {}
///`reset()` method sets EXTMEM_REJECT_INT_ST to value 0
impl crate::Resettable for EXTMEM_REJECT_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
