///Register `STATUS4` reader
pub type R = crate::R<STATUS4_SPEC>;
///Field `HFM_BITSTREAM` reader - the hufman bitstream during encoding process
pub type HFM_BITSTREAM_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - the hufman bitstream during encoding process
    #[inline(always)]
    pub fn hfm_bitstream(&self) -> HFM_BITSTREAM_R {
        HFM_BITSTREAM_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS4")
            .field("hfm_bitstream", &self.hfm_bitstream())
            .finish()
    }
}
/**Trace and Debug registers

You can [`read`](crate::generic::Reg::read) this register and get [`status4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS4_SPEC;
impl crate::RegisterSpec for STATUS4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status4::R`](R) reader structure
impl crate::Readable for STATUS4_SPEC {}
///`reset()` method sets STATUS4 to value 0
impl crate::Resettable for STATUS4_SPEC {
    const RESET_VALUE: u32 = 0;
}
