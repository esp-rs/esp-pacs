///Register `_0_TXLINK_DSCR_BF1` reader
pub type R = crate::R<_0_TXLINK_DSCR_BF1_SPEC>;
///Field `SLC0_TXLINK_DSCR_BF1` reader -
pub type SLC0_TXLINK_DSCR_BF1_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31
    #[inline(always)]
    pub fn slc0_txlink_dscr_bf1(&self) -> SLC0_TXLINK_DSCR_BF1_R {
        SLC0_TXLINK_DSCR_BF1_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_TXLINK_DSCR_BF1")
            .field("slc0_txlink_dscr_bf1", &self.slc0_txlink_dscr_bf1())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`_0_txlink_dscr_bf1::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct _0_TXLINK_DSCR_BF1_SPEC;
impl crate::RegisterSpec for _0_TXLINK_DSCR_BF1_SPEC {
    type Ux = u32;
}
///`read()` method returns [`_0_txlink_dscr_bf1::R`](R) reader structure
impl crate::Readable for _0_TXLINK_DSCR_BF1_SPEC {}
///`reset()` method sets _0_TXLINK_DSCR_BF1 to value 0
impl crate::Resettable for _0_TXLINK_DSCR_BF1_SPEC {
    const RESET_VALUE: u32 = 0;
}
