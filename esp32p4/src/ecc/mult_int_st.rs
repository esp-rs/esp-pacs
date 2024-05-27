///Register `MULT_INT_ST` reader
pub type R = crate::R<MULT_INT_ST_SPEC>;
///Field `CALC_DONE` reader - The masked interrupt status bit for the ecc_calc_done_int interrupt
pub type CALC_DONE_R = crate::BitReader;
impl R {
    ///Bit 0 - The masked interrupt status bit for the ecc_calc_done_int interrupt
    #[inline(always)]
    pub fn calc_done(&self) -> CALC_DONE_R {
        CALC_DONE_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MULT_INT_ST")
            .field("calc_done", &self.calc_done())
            .finish()
    }
}
/**ECC interrupt status register.

You can [`read`](crate::generic::Reg::read) this register and get [`mult_int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct MULT_INT_ST_SPEC;
impl crate::RegisterSpec for MULT_INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`mult_int_st::R`](R) reader structure
impl crate::Readable for MULT_INT_ST_SPEC {}
///`reset()` method sets MULT_INT_ST to value 0
impl crate::Resettable for MULT_INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
