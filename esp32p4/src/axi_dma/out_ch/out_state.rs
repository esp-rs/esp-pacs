///Register `OUT_STATE` reader
pub type R = crate::R<OUT_STATE_SPEC>;
///Field `OUTLINK_DSCR_ADDR` reader - This register stores the current outlink descriptor's address.
pub type OUTLINK_DSCR_ADDR_R = crate::FieldReader<u32>;
///Field `OUT_DSCR_STATE` reader - reserved
pub type OUT_DSCR_STATE_R = crate::FieldReader;
///Field `OUT_STATE` reader - reserved
pub type OUT_STATE_R = crate::FieldReader;
impl R {
    ///Bits 0:17 - This register stores the current outlink descriptor's address.
    #[inline(always)]
    pub fn outlink_dscr_addr(&self) -> OUTLINK_DSCR_ADDR_R {
        OUTLINK_DSCR_ADDR_R::new(self.bits & 0x0003_ffff)
    }
    ///Bits 18:19 - reserved
    #[inline(always)]
    pub fn out_dscr_state(&self) -> OUT_DSCR_STATE_R {
        OUT_DSCR_STATE_R::new(((self.bits >> 18) & 3) as u8)
    }
    ///Bits 20:22 - reserved
    #[inline(always)]
    pub fn out_state(&self) -> OUT_STATE_R {
        OUT_STATE_R::new(((self.bits >> 20) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_STATE")
            .field("outlink_dscr_addr", &self.outlink_dscr_addr())
            .field("out_dscr_state", &self.out_dscr_state())
            .field("out_state", &self.out_state())
            .finish()
    }
}
/**Transmit status of Tx channel0

You can [`read`](crate::generic::Reg::read) this register and get [`out_state::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct OUT_STATE_SPEC;
impl crate::RegisterSpec for OUT_STATE_SPEC {
    type Ux = u32;
}
///`read()` method returns [`out_state::R`](R) reader structure
impl crate::Readable for OUT_STATE_SPEC {}
///`reset()` method sets OUT_STATE to value 0
impl crate::Resettable for OUT_STATE_SPEC {
    const RESET_VALUE: u32 = 0;
}
