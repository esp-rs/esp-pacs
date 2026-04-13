#[doc = "Register `USB_OTGHS_PHY_ST` reader"]
pub type R = crate::R<USB_OTGHS_PHY_ST_SPEC>;
#[doc = "Field `USB_SOFT_RESET_ACTV_PDOMAIN` reader - Todo"]
pub type USB_SOFT_RESET_ACTV_PDOMAIN_R = crate::BitReader;
#[doc = "Field `UTMISRP_SESSEND` reader - Todo"]
pub type UTMISRP_SESSEND_R = crate::BitReader;
#[doc = "Field `UTMIOTG_VBUSVALID` reader - Todo"]
pub type UTMIOTG_VBUSVALID_R = crate::BitReader;
#[doc = "Field `UTMISRP_BVALID` reader - Todo"]
pub type UTMISRP_BVALID_R = crate::BitReader;
#[doc = "Field `UTMISRP_SESSVALID` reader - Todo"]
pub type UTMISRP_SESSVALID_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Todo"]
    #[inline(always)]
    pub fn usb_soft_reset_actv_pdomain(&self) -> USB_SOFT_RESET_ACTV_PDOMAIN_R {
        USB_SOFT_RESET_ACTV_PDOMAIN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Todo"]
    #[inline(always)]
    pub fn utmisrp_sessend(&self) -> UTMISRP_SESSEND_R {
        UTMISRP_SESSEND_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Todo"]
    #[inline(always)]
    pub fn utmiotg_vbusvalid(&self) -> UTMIOTG_VBUSVALID_R {
        UTMIOTG_VBUSVALID_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Todo"]
    #[inline(always)]
    pub fn utmisrp_bvalid(&self) -> UTMISRP_BVALID_R {
        UTMISRP_BVALID_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Todo"]
    #[inline(always)]
    pub fn utmisrp_sessvalid(&self) -> UTMISRP_SESSVALID_R {
        UTMISRP_SESSVALID_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_OTGHS_PHY_ST")
            .field(
                "usb_soft_reset_actv_pdomain",
                &self.usb_soft_reset_actv_pdomain(),
            )
            .field("utmisrp_sessend", &self.utmisrp_sessend())
            .field("utmiotg_vbusvalid", &self.utmiotg_vbusvalid())
            .field("utmisrp_bvalid", &self.utmisrp_bvalid())
            .field("utmisrp_sessvalid", &self.utmisrp_sessvalid())
            .finish()
    }
}
#[doc = "Usb otg2.0 PHY status register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_otghs_phy_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_OTGHS_PHY_ST_SPEC;
impl crate::RegisterSpec for USB_OTGHS_PHY_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_otghs_phy_st::R`](R) reader structure"]
impl crate::Readable for USB_OTGHS_PHY_ST_SPEC {}
#[doc = "`reset()` method sets USB_OTGHS_PHY_ST to value 0"]
impl crate::Resettable for USB_OTGHS_PHY_ST_SPEC {}
