///Register `FSM_STATUS` reader
pub type R = crate::R<FSM_STATUS_SPEC>;
///Field `ST_URX_OUT` reader - This is the status register of receiver.
pub type ST_URX_OUT_R = crate::FieldReader;
///Field `ST_UTX_OUT` reader - This is the status register of transmitter.
pub type ST_UTX_OUT_R = crate::FieldReader;
impl R {
    ///Bits 0:3 - This is the status register of receiver.
    #[inline(always)]
    pub fn st_urx_out(&self) -> ST_URX_OUT_R {
        ST_URX_OUT_R::new((self.bits & 0x0f) as u8)
    }
    ///Bits 4:7 - This is the status register of transmitter.
    #[inline(always)]
    pub fn st_utx_out(&self) -> ST_UTX_OUT_R {
        ST_UTX_OUT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FSM_STATUS")
            .field("st_urx_out", &self.st_urx_out())
            .field("st_utx_out", &self.st_utx_out())
            .finish()
    }
}
/**UART transmit and receive status.

You can [`read`](crate::generic::Reg::read) this register and get [`fsm_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct FSM_STATUS_SPEC;
impl crate::RegisterSpec for FSM_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`fsm_status::R`](R) reader structure
impl crate::Readable for FSM_STATUS_SPEC {}
///`reset()` method sets FSM_STATUS to value 0
impl crate::Resettable for FSM_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
