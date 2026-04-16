#[doc = "Register `HP_USB_OTGHS_PHY_CTRL` reader"]
pub type R = crate::R<HP_USB_OTGHS_PHY_CTRL_SPEC>;
#[doc = "Register `HP_USB_OTGHS_PHY_CTRL` writer"]
pub type W = crate::W<HP_USB_OTGHS_PHY_CTRL_SPEC>;
#[doc = "Field `HP_UTMIOTG_IDPULLUP` reader - Set this bit to pull up USB OTG2.0 PHY id"]
pub type HP_UTMIOTG_IDPULLUP_R = crate::BitReader;
#[doc = "Field `HP_UTMIOTG_IDPULLUP` writer - Set this bit to pull up USB OTG2.0 PHY id"]
pub type HP_UTMIOTG_IDPULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMIOTG_DPPULLDOWN` reader - Set this bit to pull down USB OTG2.0 PHY dp"]
pub type HP_UTMIOTG_DPPULLDOWN_R = crate::BitReader;
#[doc = "Field `HP_UTMIOTG_DPPULLDOWN` writer - Set this bit to pull down USB OTG2.0 PHY dp"]
pub type HP_UTMIOTG_DPPULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMIOTG_DMPULLDOWN` reader - Set this bit to pull down USB OTG2.0 PHY dm"]
pub type HP_UTMIOTG_DMPULLDOWN_R = crate::BitReader;
#[doc = "Field `HP_UTMIOTG_DMPULLDOWN` writer - Set this bit to pull down USB OTG2.0 PHY dm"]
pub type HP_UTMIOTG_DMPULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMISRP_CHRGVBUS` reader - Set this bit to charge USB OTG2.0 PHY vbus"]
pub type HP_UTMISRP_CHRGVBUS_R = crate::BitReader;
#[doc = "Field `HP_UTMISRP_CHRGVBUS` writer - Set this bit to charge USB OTG2.0 PHY vbus"]
pub type HP_UTMISRP_CHRGVBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HP_UTMISRP_DISCHRGVBUS` reader - Set this bit to discharge USB OTG2.0 PHY vbus"]
pub type HP_UTMISRP_DISCHRGVBUS_R = crate::BitReader;
#[doc = "Field `HP_UTMISRP_DISCHRGVBUS` writer - Set this bit to discharge USB OTG2.0 PHY vbus"]
pub type HP_UTMISRP_DISCHRGVBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to pull up USB OTG2.0 PHY id"]
    #[inline(always)]
    pub fn hp_utmiotg_idpullup(&self) -> HP_UTMIOTG_IDPULLUP_R {
        HP_UTMIOTG_IDPULLUP_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to pull down USB OTG2.0 PHY dp"]
    #[inline(always)]
    pub fn hp_utmiotg_dppulldown(&self) -> HP_UTMIOTG_DPPULLDOWN_R {
        HP_UTMIOTG_DPPULLDOWN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to pull down USB OTG2.0 PHY dm"]
    #[inline(always)]
    pub fn hp_utmiotg_dmpulldown(&self) -> HP_UTMIOTG_DMPULLDOWN_R {
        HP_UTMIOTG_DMPULLDOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to charge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_chrgvbus(&self) -> HP_UTMISRP_CHRGVBUS_R {
        HP_UTMISRP_CHRGVBUS_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Set this bit to discharge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_dischrgvbus(&self) -> HP_UTMISRP_DISCHRGVBUS_R {
        HP_UTMISRP_DISCHRGVBUS_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_USB_OTGHS_PHY_CTRL")
            .field("hp_utmiotg_idpullup", &self.hp_utmiotg_idpullup())
            .field("hp_utmiotg_dppulldown", &self.hp_utmiotg_dppulldown())
            .field("hp_utmiotg_dmpulldown", &self.hp_utmiotg_dmpulldown())
            .field("hp_utmisrp_chrgvbus", &self.hp_utmisrp_chrgvbus())
            .field("hp_utmisrp_dischrgvbus", &self.hp_utmisrp_dischrgvbus())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to pull up USB OTG2.0 PHY id"]
    #[inline(always)]
    pub fn hp_utmiotg_idpullup(&mut self) -> HP_UTMIOTG_IDPULLUP_W<'_, HP_USB_OTGHS_PHY_CTRL_SPEC> {
        HP_UTMIOTG_IDPULLUP_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to pull down USB OTG2.0 PHY dp"]
    #[inline(always)]
    pub fn hp_utmiotg_dppulldown(
        &mut self,
    ) -> HP_UTMIOTG_DPPULLDOWN_W<'_, HP_USB_OTGHS_PHY_CTRL_SPEC> {
        HP_UTMIOTG_DPPULLDOWN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to pull down USB OTG2.0 PHY dm"]
    #[inline(always)]
    pub fn hp_utmiotg_dmpulldown(
        &mut self,
    ) -> HP_UTMIOTG_DMPULLDOWN_W<'_, HP_USB_OTGHS_PHY_CTRL_SPEC> {
        HP_UTMIOTG_DMPULLDOWN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to charge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_chrgvbus(&mut self) -> HP_UTMISRP_CHRGVBUS_W<'_, HP_USB_OTGHS_PHY_CTRL_SPEC> {
        HP_UTMISRP_CHRGVBUS_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to discharge USB OTG2.0 PHY vbus"]
    #[inline(always)]
    pub fn hp_utmisrp_dischrgvbus(
        &mut self,
    ) -> HP_UTMISRP_DISCHRGVBUS_W<'_, HP_USB_OTGHS_PHY_CTRL_SPEC> {
        HP_UTMISRP_DISCHRGVBUS_W::new(self, 4)
    }
}
#[doc = "Usb otg2.0 PHY control register\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_usb_otghs_phy_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_usb_otghs_phy_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_USB_OTGHS_PHY_CTRL_SPEC;
impl crate::RegisterSpec for HP_USB_OTGHS_PHY_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_usb_otghs_phy_ctrl::R`](R) reader structure"]
impl crate::Readable for HP_USB_OTGHS_PHY_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_usb_otghs_phy_ctrl::W`](W) writer structure"]
impl crate::Writable for HP_USB_OTGHS_PHY_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HP_USB_OTGHS_PHY_CTRL to value 0x07"]
impl crate::Resettable for HP_USB_OTGHS_PHY_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x07;
}
