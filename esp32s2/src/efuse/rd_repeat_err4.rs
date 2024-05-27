///Register `RD_REPEAT_ERR4` reader
pub type R = crate::R<RD_REPEAT_ERR4_SPEC>;
///Field `RPT4_RESERVED4_ERR` reader - If any bit in RPT4_RESERVED4 is 1, there is a programming error in EFUSE_RPT4_RESERVED4.
pub type RPT4_RESERVED4_ERR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:23 - If any bit in RPT4_RESERVED4 is 1, there is a programming error in EFUSE_RPT4_RESERVED4.
    #[inline(always)]
    pub fn rpt4_reserved4_err(&self) -> RPT4_RESERVED4_ERR_R {
        RPT4_RESERVED4_ERR_R::new(self.bits & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_REPEAT_ERR4")
            .field("rpt4_reserved4_err", &self.rpt4_reserved4_err())
            .finish()
    }
}
/**Programming error record register 4 of BLOCK0.

You can [`read`](crate::generic::Reg::read) this register and get [`rd_repeat_err4::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct RD_REPEAT_ERR4_SPEC;
impl crate::RegisterSpec for RD_REPEAT_ERR4_SPEC {
    type Ux = u32;
}
///`read()` method returns [`rd_repeat_err4::R`](R) reader structure
impl crate::Readable for RD_REPEAT_ERR4_SPEC {}
///`reset()` method sets RD_REPEAT_ERR4 to value 0
impl crate::Resettable for RD_REPEAT_ERR4_SPEC {
    const RESET_VALUE: u32 = 0;
}
