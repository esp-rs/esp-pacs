///Register `TIMING_CALI` reader
pub type R = crate::R<TIMING_CALI_SPEC>;
///Field `TIMING_CLK_ENA` reader - The bit is used to enable timing adjust clock for all reading operations.
pub type TIMING_CLK_ENA_R = crate::BitReader;
///Field `TIMING_CALI` reader - The bit is used to enable timing auto-calibration for all reading operations.
pub type TIMING_CALI_R = crate::BitReader;
///Field `EXTRA_DUMMY_CYCLELEN` reader - add extra dummy spi clock cycle length for spi clock calibration.
pub type EXTRA_DUMMY_CYCLELEN_R = crate::FieldReader;
impl R {
    ///Bit 0 - The bit is used to enable timing adjust clock for all reading operations.
    #[inline(always)]
    pub fn timing_clk_ena(&self) -> TIMING_CLK_ENA_R {
        TIMING_CLK_ENA_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The bit is used to enable timing auto-calibration for all reading operations.
    #[inline(always)]
    pub fn timing_cali(&self) -> TIMING_CALI_R {
        TIMING_CALI_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bits 2:4 - add extra dummy spi clock cycle length for spi clock calibration.
    #[inline(always)]
    pub fn extra_dummy_cyclelen(&self) -> EXTRA_DUMMY_CYCLELEN_R {
        EXTRA_DUMMY_CYCLELEN_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMING_CALI")
            .field("timing_clk_ena", &self.timing_clk_ena())
            .field("timing_cali", &self.timing_cali())
            .field("extra_dummy_cyclelen", &self.extra_dummy_cyclelen())
            .finish()
    }
}
/**SPI0 timing calibration register

You can [`read`](crate::generic::Reg::read) this register and get [`timing_cali::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct TIMING_CALI_SPEC;
impl crate::RegisterSpec for TIMING_CALI_SPEC {
    type Ux = u32;
}
///`read()` method returns [`timing_cali::R`](R) reader structure
impl crate::Readable for TIMING_CALI_SPEC {}
///`reset()` method sets TIMING_CALI to value 0
impl crate::Resettable for TIMING_CALI_SPEC {
    const RESET_VALUE: u32 = 0;
}
