///Register `DSCR` reader
pub type R = crate::R<DSCR_SPEC>;
///Field `INLINK_DSCR` reader - The address of the next inlink descriptor address x.
pub type INLINK_DSCR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - The address of the next inlink descriptor address x.
    #[inline(always)]
    pub fn inlink_dscr(&self) -> INLINK_DSCR_R {
        INLINK_DSCR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DSCR")
            .field("inlink_dscr", &self.inlink_dscr())
            .finish()
    }
}
/**RX CHx next dscr addr register

You can [`read`](crate::generic::Reg::read) this register and get [`dscr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct DSCR_SPEC;
impl crate::RegisterSpec for DSCR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`dscr::R`](R) reader structure
impl crate::Readable for DSCR_SPEC {}
///`reset()` method sets DSCR to value 0
impl crate::Resettable for DSCR_SPEC {
    const RESET_VALUE: u32 = 0;
}
