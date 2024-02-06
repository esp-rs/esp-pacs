#[doc = "Register `LP_ADDRHOLE_INFO` reader"]
pub type R = crate::R<LP_ADDRHOLE_INFO_SPEC>;
#[doc = "Field `LP_ADDRHOLE_ID` reader - master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma."]
pub type LP_ADDRHOLE_ID_R = crate::FieldReader;
#[doc = "Field `LP_ADDRHOLE_WR` reader - 1:write trans, 0: read trans."]
pub type LP_ADDRHOLE_WR_R = crate::BitReader;
#[doc = "Field `LP_ADDRHOLE_SECURE` reader - 1: illegal address access, 0: access without permission"]
pub type LP_ADDRHOLE_SECURE_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:4 - master id: 5'h0: hp core0, 5'h1:hp core1, 5'h2:lp core, 5'h3:usb otg11, 5'h4: regdma, 5'h5: gmac, 5'h5 sdmmc, 5'h7: usbotg20, 5'h8: trace0, 5'h9: trace1, 5'ha tcm monitor, 5'hb: l2mem monitor. 5'h10~5'h1f: ahb pdma."]
    #[inline(always)]
    pub fn lp_addrhole_id(&self) -> LP_ADDRHOLE_ID_R {
        LP_ADDRHOLE_ID_R::new((self.bits & 0x1f) as u8)
    }
    #[doc = "Bit 5 - 1:write trans, 0: read trans."]
    #[inline(always)]
    pub fn lp_addrhole_wr(&self) -> LP_ADDRHOLE_WR_R {
        LP_ADDRHOLE_WR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - 1: illegal address access, 0: access without permission"]
    #[inline(always)]
    pub fn lp_addrhole_secure(&self) -> LP_ADDRHOLE_SECURE_R {
        LP_ADDRHOLE_SECURE_R::new(((self.bits >> 6) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LP_ADDRHOLE_INFO")
            .field(
                "lp_addrhole_id",
                &format_args!("{}", self.lp_addrhole_id().bits()),
            )
            .field(
                "lp_addrhole_wr",
                &format_args!("{}", self.lp_addrhole_wr().bit()),
            )
            .field(
                "lp_addrhole_secure",
                &format_args!("{}", self.lp_addrhole_secure().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LP_ADDRHOLE_INFO_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`lp_addrhole_info::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct LP_ADDRHOLE_INFO_SPEC;
impl crate::RegisterSpec for LP_ADDRHOLE_INFO_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`lp_addrhole_info::R`](R) reader structure"]
impl crate::Readable for LP_ADDRHOLE_INFO_SPEC {}
#[doc = "`reset()` method sets LP_ADDRHOLE_INFO to value 0"]
impl crate::Resettable for LP_ADDRHOLE_INFO_SPEC {
    const RESET_VALUE: u32 = 0;
}
