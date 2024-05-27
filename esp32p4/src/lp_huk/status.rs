///Register `STATUS` reader
pub type R = crate::R<STATUS_SPEC>;
///Field `STATUS` reader - The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved.
pub type STATUS_R = crate::FieldReader;
///Field `RISK_LEVEL` reader - The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid.
pub type RISK_LEVEL_R = crate::FieldReader;
impl R {
    ///Bits 0:1 - The HUK generation status. 0: HUK is not generated. 1: HUK is generated and valid. 2: HUK is generated but invalid. 3: reserved.
    #[inline(always)]
    pub fn status(&self) -> STATUS_R {
        STATUS_R::new((self.bits & 3) as u8)
    }
    ///Bits 2:4 - The risk level of HUK. 0-6: the higher the risk level is, the more error bits there are in the PUF SRAM. 7: Error Level, HUK is invalid.
    #[inline(always)]
    pub fn risk_level(&self) -> RISK_LEVEL_R {
        RISK_LEVEL_R::new(((self.bits >> 2) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("STATUS")
            .field("status", &self.status())
            .field("risk_level", &self.risk_level())
            .finish()
    }
}
/**HUK Generator HUK status register

You can [`read`](crate::generic::Reg::read) this register and get [`status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct STATUS_SPEC;
impl crate::RegisterSpec for STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`status::R`](R) reader structure
impl crate::Readable for STATUS_SPEC {}
///`reset()` method sets STATUS to value 0
impl crate::Resettable for STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
