///Register `USB_CTRL` reader
pub type R = crate::R<USB_CTRL_SPEC>;
///Register `USB_CTRL` writer
pub type W = crate::W<USB_CTRL_SPEC>;
///Field `SW_HW_USB_PHY_SEL` reader - need_des
pub type SW_HW_USB_PHY_SEL_R = crate::BitReader;
///Field `SW_HW_USB_PHY_SEL` writer - need_des
pub type SW_HW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `SW_USB_PHY_SEL` reader - need_des
pub type SW_USB_PHY_SEL_R = crate::BitReader;
///Field `SW_USB_PHY_SEL` writer - need_des
pub type SW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBOTG20_WAKEUP_CLR` writer - clear usb wakeup to PMU.
pub type USBOTG20_WAKEUP_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `USBOTG20_IN_SUSPEND` reader - indicate usb otg2.0 is in suspend state.
pub type USBOTG20_IN_SUSPEND_R = crate::BitReader;
///Field `USBOTG20_IN_SUSPEND` writer - indicate usb otg2.0 is in suspend state.
pub type USBOTG20_IN_SUSPEND_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - need_des
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 3 - indicate usb otg2.0 is in suspend state.
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
    ///Bit 0 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W<USB_CTRL_SPEC> {
        SW_HW_USB_PHY_SEL_W::new(self, 0)
    }
    ///Bit 1 - need_des
    #[inline(always)]
    #[must_use]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W<USB_CTRL_SPEC> {
        SW_USB_PHY_SEL_W::new(self, 1)
    }
    ///Bit 2 - clear usb wakeup to PMU.
    #[inline(always)]
    #[must_use]
    pub fn usbotg20_wakeup_clr(&mut self) -> USBOTG20_WAKEUP_CLR_W<USB_CTRL_SPEC> {
        USBOTG20_WAKEUP_CLR_W::new(self, 2)
    }
    ///Bit 3 - indicate usb otg2.0 is in suspend state.
    #[inline(always)]
    #[must_use]
    pub fn usbotg20_in_suspend(&mut self) -> USBOTG20_IN_SUSPEND_W<USB_CTRL_SPEC> {
        USBOTG20_IN_SUSPEND_W::new(self, 3)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`usb_ctrl::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_ctrl::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct USB_CTRL_SPEC;
impl crate::RegisterSpec for USB_CTRL_SPEC {
    type Ux = u32;
}
///`read()` method returns [`usb_ctrl::R`](R) reader structure
impl crate::Readable for USB_CTRL_SPEC {}
///`write(|w| ..)` method takes [`usb_ctrl::W`](W) writer structure
impl crate::Writable for USB_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets USB_CTRL to value 0
impl crate::Resettable for USB_CTRL_SPEC {
    const RESET_VALUE: u32 = 0;
}
