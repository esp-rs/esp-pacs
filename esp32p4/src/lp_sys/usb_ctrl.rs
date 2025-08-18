#[doc = "Register `USB_CTRL` reader"]
pub type R = crate::R<USB_CTRL_SPEC>;
#[doc = "Register `USB_CTRL` writer"]
pub type W = crate::W<USB_CTRL_SPEC>;
#[doc = "Field `SW_HW_USB_PHY_SEL` reader - need_des"]
pub type SW_HW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_HW_USB_PHY_SEL` writer - need_des"]
pub type SW_HW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_USB_PHY_SEL` reader - need_des"]
pub type SW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_USB_PHY_SEL` writer - need_des"]
pub type SW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOTG20_WAKEUP_CLR` writer - clear usb wakeup to PMU."]
pub type USBOTG20_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USBOTG20_IN_SUSPEND` reader - indicate usb otg2.0 is in suspend state."]
pub type USBOTG20_IN_SUSPEND_R = crate::BitReader;
#[doc = "Field `USBOTG20_IN_SUSPEND` writer - indicate usb otg2.0 is in suspend state."]
pub type USBOTG20_IN_SUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3 - indicate usb otg2.0 is in suspend state."]
    #[inline(always)]
    pub fn usbotg20_in_suspend(&self) -> USBOTG20_IN_SUSPEND_R {
        USBOTG20_IN_SUSPEND_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_CTRL")
            .field("sw_hw_usb_phy_sel", &self.sw_hw_usb_phy_sel())
            .field("sw_usb_phy_sel", &self.sw_usb_phy_sel())
            .field("usbotg20_in_suspend", &self.usbotg20_in_suspend())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W<'_, USB_CTRL_SPEC> {
        SW_HW_USB_PHY_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W<'_, USB_CTRL_SPEC> {
        SW_USB_PHY_SEL_W::new(self, 1)
    }
    #[doc = "Bit 2 - clear usb wakeup to PMU."]
    #[inline(always)]
    pub fn usbotg20_wakeup_clr(&mut self) -> USBOTG20_WAKEUP_CLR_W<'_, USB_CTRL_SPEC> {
        USBOTG20_WAKEUP_CLR_W::new(self, 2)
    }
    #[doc = "Bit 3 - indicate usb otg2.0 is in suspend state."]
    #[inline(always)]
    pub fn usbotg20_in_suspend(&mut self) -> USBOTG20_IN_SUSPEND_W<'_, USB_CTRL_SPEC> {
        USBOTG20_IN_SUSPEND_W::new(self, 3)
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
