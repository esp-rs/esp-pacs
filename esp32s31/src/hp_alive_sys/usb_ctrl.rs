#[doc = "Register `USB_CTRL` reader"]
pub type R = crate::R<USB_CTRL_SPEC>;
#[doc = "Register `USB_CTRL` writer"]
pub type W = crate::W<USB_CTRL_SPEC>;
#[doc = "Field `USB_OTGHS_PHY_CHRGVBUS` reader - phy chrgvbus"]
pub type USB_OTGHS_PHY_CHRGVBUS_R = crate::BitReader;
#[doc = "Field `USB_OTGHS_PHY_CHRGVBUS` writer - phy chrgvbus"]
pub type USB_OTGHS_PHY_CHRGVBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTGHS_PHY_DISCHRGVBUS` reader - phy dischrgvbus"]
pub type USB_OTGHS_PHY_DISCHRGVBUS_R = crate::BitReader;
#[doc = "Field `USB_OTGHS_PHY_DISCHRGVBUS` writer - phy dischrgvbus"]
pub type USB_OTGHS_PHY_DISCHRGVBUS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTGHS_PHY_DMPULLDOWN` reader - phy dmpulldown"]
pub type USB_OTGHS_PHY_DMPULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_OTGHS_PHY_DMPULLDOWN` writer - phy dmpulldown"]
pub type USB_OTGHS_PHY_DMPULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTGHS_PHY_DPPULLDOWN` reader - phy dppulldown"]
pub type USB_OTGHS_PHY_DPPULLDOWN_R = crate::BitReader;
#[doc = "Field `USB_OTGHS_PHY_DPPULLDOWN` writer - phy dppulldown"]
pub type USB_OTGHS_PHY_DPPULLDOWN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_OTGHS_PHY_IDPULLUP` reader - phy idpullup"]
pub type USB_OTGHS_PHY_IDPULLUP_R = crate::BitReader;
#[doc = "Field `USB_OTGHS_PHY_IDPULLUP` writer - phy idpullup"]
pub type USB_OTGHS_PHY_IDPULLUP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - phy chrgvbus"]
    #[inline(always)]
    pub fn usb_otghs_phy_chrgvbus(&self) -> USB_OTGHS_PHY_CHRGVBUS_R {
        USB_OTGHS_PHY_CHRGVBUS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - phy dischrgvbus"]
    #[inline(always)]
    pub fn usb_otghs_phy_dischrgvbus(&self) -> USB_OTGHS_PHY_DISCHRGVBUS_R {
        USB_OTGHS_PHY_DISCHRGVBUS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - phy dmpulldown"]
    #[inline(always)]
    pub fn usb_otghs_phy_dmpulldown(&self) -> USB_OTGHS_PHY_DMPULLDOWN_R {
        USB_OTGHS_PHY_DMPULLDOWN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - phy dppulldown"]
    #[inline(always)]
    pub fn usb_otghs_phy_dppulldown(&self) -> USB_OTGHS_PHY_DPPULLDOWN_R {
        USB_OTGHS_PHY_DPPULLDOWN_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - phy idpullup"]
    #[inline(always)]
    pub fn usb_otghs_phy_idpullup(&self) -> USB_OTGHS_PHY_IDPULLUP_R {
        USB_OTGHS_PHY_IDPULLUP_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_CTRL")
            .field("usb_otghs_phy_chrgvbus", &self.usb_otghs_phy_chrgvbus())
            .field(
                "usb_otghs_phy_dischrgvbus",
                &self.usb_otghs_phy_dischrgvbus(),
            )
            .field("usb_otghs_phy_dmpulldown", &self.usb_otghs_phy_dmpulldown())
            .field("usb_otghs_phy_dppulldown", &self.usb_otghs_phy_dppulldown())
            .field("usb_otghs_phy_idpullup", &self.usb_otghs_phy_idpullup())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - phy chrgvbus"]
    #[inline(always)]
    pub fn usb_otghs_phy_chrgvbus(&mut self) -> USB_OTGHS_PHY_CHRGVBUS_W<'_, USB_CTRL_SPEC> {
        USB_OTGHS_PHY_CHRGVBUS_W::new(self, 0)
    }
    #[doc = "Bit 1 - phy dischrgvbus"]
    #[inline(always)]
    pub fn usb_otghs_phy_dischrgvbus(&mut self) -> USB_OTGHS_PHY_DISCHRGVBUS_W<'_, USB_CTRL_SPEC> {
        USB_OTGHS_PHY_DISCHRGVBUS_W::new(self, 1)
    }
    #[doc = "Bit 2 - phy dmpulldown"]
    #[inline(always)]
    pub fn usb_otghs_phy_dmpulldown(&mut self) -> USB_OTGHS_PHY_DMPULLDOWN_W<'_, USB_CTRL_SPEC> {
        USB_OTGHS_PHY_DMPULLDOWN_W::new(self, 2)
    }
    #[doc = "Bit 3 - phy dppulldown"]
    #[inline(always)]
    pub fn usb_otghs_phy_dppulldown(&mut self) -> USB_OTGHS_PHY_DPPULLDOWN_W<'_, USB_CTRL_SPEC> {
        USB_OTGHS_PHY_DPPULLDOWN_W::new(self, 3)
    }
    #[doc = "Bit 4 - phy idpullup"]
    #[inline(always)]
    pub fn usb_otghs_phy_idpullup(&mut self) -> USB_OTGHS_PHY_IDPULLUP_W<'_, USB_CTRL_SPEC> {
        USB_OTGHS_PHY_IDPULLUP_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CTRL_SPEC;
impl crate::RegisterSpec for USB_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_ctrl::R`](R) reader structure"]
impl crate::Readable for USB_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_ctrl::W`](W) writer structure"]
impl crate::Writable for USB_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_CTRL to value 0"]
impl crate::Resettable for USB_CTRL_SPEC {}
