///Register `LP_ADDRHOLE_ADDR` reader
pub type R = crate::R<LP_ADDRHOLE_ADDR_SPEC>;
///Field `LP_ADDRHOLE_ADDR` reader - need_des
pub type LP_ADDRHOLE_ADDR_R = crate::FieldReader<u32>;
impl R {
    ///Bits 0:31 - need_des
    #[inline(always)]
    pub fn lp_addrhole_addr(&self) -> LP_ADDRHOLE_ADDR_R {
        LP_ADDRHOLE_ADDR_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ADDRHOLE_ADDR")
            .field("lp_addrhole_addr", &self.lp_addrhole_addr())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_addrhole_addr::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_ADDRHOLE_ADDR_SPEC;
impl crate::RegisterSpec for LP_ADDRHOLE_ADDR_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_addrhole_addr::R`](R) reader structure
impl crate::Readable for LP_ADDRHOLE_ADDR_SPEC {}
///`reset()` method sets LP_ADDRHOLE_ADDR to value 0
impl crate::Resettable for LP_ADDRHOLE_ADDR_SPEC {
    const RESET_VALUE: u32 = 0;
}
