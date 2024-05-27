///Register `LP_ADDRHOLE_INFO` reader
pub type R = crate::R<LP_ADDRHOLE_INFO_SPEC>;
///Field `LP_ADDRHOLE_ID` reader - master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma.
pub type LP_ADDRHOLE_ID_R = crate::FieldReader;
///Field `LP_ADDRHOLE_WR` reader - 1:write trans, 0: read trans.
pub type LP_ADDRHOLE_WR_R = crate::BitReader;
///Field `LP_ADDRHOLE_SECURE` reader - 1: illegal address access, 0: access without permission
pub type LP_ADDRHOLE_SECURE_R = crate::BitReader;
impl R {
    ///Bits 0:4 - master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma.
    #[inline(always)]
    pub fn lp_addrhole_id(&self) -> LP_ADDRHOLE_ID_R {
        LP_ADDRHOLE_ID_R::new((self.bits & 0x1f) as u8)
    }
    ///Bit 5 - 1:write trans, 0: read trans.
    #[inline(always)]
    pub fn lp_addrhole_wr(&self) -> LP_ADDRHOLE_WR_R {
        LP_ADDRHOLE_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    ///Bit 6 - 1: illegal address access, 0: access without permission
    #[inline(always)]
    pub fn lp_addrhole_secure(&self) -> LP_ADDRHOLE_SECURE_R {
        LP_ADDRHOLE_SECURE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ADDRHOLE_INFO")
            .field("lp_addrhole_id", &self.lp_addrhole_id())
            .field("lp_addrhole_wr", &self.lp_addrhole_wr())
            .field("lp_addrhole_secure", &self.lp_addrhole_secure())
            .finish()
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`lp_addrhole_info::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LP_ADDRHOLE_INFO_SPEC;
impl crate::RegisterSpec for LP_ADDRHOLE_INFO_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lp_addrhole_info::R`](R) reader structure
impl crate::Readable for LP_ADDRHOLE_INFO_SPEC {}
///`reset()` method sets LP_ADDRHOLE_INFO to value 0
impl crate::Resettable for LP_ADDRHOLE_INFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
