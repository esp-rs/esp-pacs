#[doc = "Register `USB_CTRL` reader"]
pub type R = crate::R<USB_CTRL_SPEC>;
#[doc = "Register `USB_CTRL` writer"]
pub type W = crate::W<USB_CTRL_SPEC>;
#[doc = "Field `SW_HW_USB_PHY_SEL` reader - "]
pub type SW_HW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_HW_USB_PHY_SEL` writer - "]
pub type SW_HW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_USB_PHY_SEL` reader - "]
pub type SW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_USB_PHY_SEL` writer - "]
pub type SW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOTGHS_WAKEUP_CLR` writer - "]
pub type USBOTGHS_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOTGHS_IN_SUSPEND` reader - "]
pub type USBOTGHS_IN_SUSPEND_R = crate::BitReader;
#[doc = "Field `USBOTGHS_IN_SUSPEND` writer - "]
pub type USBOTGHS_IN_SUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOTGHS_LS_MODE` reader - "]
pub type USBOTGHS_LS_MODE_R = crate::BitReader;
#[doc = "Field `USBOTGHS_LS_MODE` writer - "]
pub type USBOTGHS_LS_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbotghs_in_suspend(&self) -> USBOTGHS_IN_SUSPEND_R {
        USBOTGHS_IN_SUSPEND_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbotghs_ls_mode(&self) -> USBOTGHS_LS_MODE_R {
        USBOTGHS_LS_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_CTRL")
            .field("sw_hw_usb_phy_sel", &self.sw_hw_usb_phy_sel())
            .field("sw_usb_phy_sel", &self.sw_usb_phy_sel())
            .field("usbotghs_in_suspend", &self.usbotghs_in_suspend())
            .field("usbotghs_ls_mode", &self.usbotghs_ls_mode())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W<'_, USB_CTRL_SPEC> {
        SW_HW_USB_PHY_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W<'_, USB_CTRL_SPEC> {
        SW_USB_PHY_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn usbotghs_wakeup_clr(&mut self) -> USBOTGHS_WAKEUP_CLR_W<'_, USB_CTRL_SPEC> {
        USBOTGHS_WAKEUP_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn usbotghs_in_suspend(&mut self) -> USBOTGHS_IN_SUSPEND_W<'_, USB_CTRL_SPEC> {
        USBOTGHS_IN_SUSPEND_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn usbotghs_ls_mode(&mut self) -> USBOTGHS_LS_MODE_W<'_, USB_CTRL_SPEC> {
        USBOTGHS_LS_MODE_W::new(self, 4)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
