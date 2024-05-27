///Register `IDBUS_ADDRHOLE_ADDR` reader
pub type R = crate::R<IDBUS_ADDRHOLE_ADDR_SPEC>;
///Field `IDBUS_ADDRHOLE_ADDR` reader - need_des
pub type IDBUS_ADDRHOLE_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn idbus_addrhole_addr(&self) -> IDBUS_ADDRHOLE_ADDR_R {
        IDBUS_ADDRHOLE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IDBUS_ADDRHOLE_ADDR")
            .field("idbus_addrhole_addr", &self.idbus_addrhole_addr())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`idbus_addrhole_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct IDBUS_ADDRHOLE_ADDR_SPEC;
impl crate::RegisterSpec for IDBUS_ADDRHOLE_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`idbus_addrhole_addr::R`](R) reader structure
impl crate::Readable for IDBUS_ADDRHOLE_ADDR_SPEC {}
///`reset()` method sets IDBUS_ADDRHOLE_ADDR to value 0
impl crate::Resettable for IDBUS_ADDRHOLE_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
