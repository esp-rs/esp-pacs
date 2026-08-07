#[doc = "Register `USB_PHY_SEL` reader"]
pub type R = crate::R<USB_PHY_SEL_SPEC>;
#[doc = "Register `USB_PHY_SEL` writer"]
pub type W = crate::W<USB_PHY_SEL_SPEC>;
#[doc = "Field `SW_HW_USB_PHY_SEL` reader - 0: the control bit of efuse will determine whether to exchange usb0/1_phy,1: LP_AON_SW_USB_PHY_SEL will determine whether to exchange usb0/1_phy"]
pub type SW_HW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_HW_USB_PHY_SEL` writer - 0: the control bit of efuse will determine whether to exchange usb0/1_phy,1: LP_AON_SW_USB_PHY_SEL will determine whether to exchange usb0/1_phy"]
pub type SW_HW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_USB_PHY_SEL` reader - when LP_AON_SW_HW_USB_PHY_SEL is configured as 1 , a register value of 0 means no swapping the phy"]
pub type SW_USB_PHY_SEL_R = crate::BitReader;
#[doc = "Field `SW_USB_PHY_SEL` writer - when LP_AON_SW_HW_USB_PHY_SEL is configured as 1 , a register value of 0 means no swapping the phy"]
pub type SW_USB_PHY_SEL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - 0: the control bit of efuse will determine whether to exchange usb0/1_phy,1: LP_AON_SW_USB_PHY_SEL will determine whether to exchange usb0/1_phy"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&self) -> SW_HW_USB_PHY_SEL_R {
        SW_HW_USB_PHY_SEL_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when LP_AON_SW_HW_USB_PHY_SEL is configured as 1 , a register value of 0 means no swapping the phy"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&self) -> SW_USB_PHY_SEL_R {
        SW_USB_PHY_SEL_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_PHY_SEL")
            .field("sw_hw_usb_phy_sel", &self.sw_hw_usb_phy_sel())
            .field("sw_usb_phy_sel", &self.sw_usb_phy_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - 0: the control bit of efuse will determine whether to exchange usb0/1_phy,1: LP_AON_SW_USB_PHY_SEL will determine whether to exchange usb0/1_phy"]
    #[inline(always)]
    pub fn sw_hw_usb_phy_sel(&mut self) -> SW_HW_USB_PHY_SEL_W<'_, USB_PHY_SEL_SPEC> {
        SW_HW_USB_PHY_SEL_W::new(self, 0)
    }
    #[doc = "Bit 1 - when LP_AON_SW_HW_USB_PHY_SEL is configured as 1 , a register value of 0 means no swapping the phy"]
    #[inline(always)]
    pub fn sw_usb_phy_sel(&mut self) -> SW_USB_PHY_SEL_W<'_, USB_PHY_SEL_SPEC> {
        SW_USB_PHY_SEL_W::new(self, 1)
    }
}
#[doc = "need des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_phy_sel::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_phy_sel::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_PHY_SEL_SPEC;
impl crate::RegisterSpec for USB_PHY_SEL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_phy_sel::R`](R) reader structure"]
impl crate::Readable for USB_PHY_SEL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_phy_sel::W`](W) writer structure"]
impl crate::Writable for USB_PHY_SEL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_PHY_SEL to value 0"]
impl crate::Resettable for USB_PHY_SEL_SPEC {}
