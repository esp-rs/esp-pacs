///Register `WRTPRT` reader
pub type R = crate::R<WRTPRT_SPEC>;
///Field `WRITE_PROTECT` reader - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented.
pub type WRITE_PROTECT_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - Value on sdhost_card_write_prt input ports (1 bit per card). 1 represents write protection. Only NUM_CARDS number of bits are implemented.
    #[inline(always)]
    pub fn write_protect(&self) -> WRITE_PROTECT_R {
        WRITE_PROTECT_R::new((self.bits & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WRTPRT")
            .field("write_protect", &self.write_protect())
            .finish()
    }
}
/**Card write protection (WP) status register

You can [`read`](crate::generic::Reg::read) this register and get [`wrtprt::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct WRTPRT_SPEC;
impl crate::RegisterSpec for WRTPRT_SPEC {
    type Ux = u32;
}
///`read()` method returns [`wrtprt::R`](R) reader structure
impl crate::Readable for WRTPRT_SPEC {}
///`reset()` method sets WRTPRT to value 0
impl crate::Resettable for WRTPRT_SPEC {
    const RESET_VALUE: u32 = 0;
}
