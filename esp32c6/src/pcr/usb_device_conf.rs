#[doc = "Register `USB_DEVICE_CONF` reader"]
pub type R = crate::R<USB_DEVICE_CONF_SPEC>;
#[doc = "Register `USB_DEVICE_CONF` writer"]
pub type W = crate::W<USB_DEVICE_CONF_SPEC>;
#[doc = "Field `USB_DEVICE_CLK_EN` reader - Set 1 to enable usb_device clock"]
pub type USB_DEVICE_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_CLK_EN` writer - Set 1 to enable usb_device clock"]
pub type USB_DEVICE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DEVICE_RST_EN` reader - Set 0 to reset usb_device module"]
pub type USB_DEVICE_RST_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_RST_EN` writer - Set 0 to reset usb_device module"]
pub type USB_DEVICE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set 1 to enable usb_device clock"]
    #[inline(always)]
    pub fn usb_device_clk_en(&self) -> USB_DEVICE_CLK_EN_R {
        USB_DEVICE_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 0 to reset usb_device module"]
    #[inline(always)]
    pub fn usb_device_rst_en(&self) -> USB_DEVICE_RST_EN_R {
        USB_DEVICE_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DEVICE_CONF")
            .field(
                "usb_device_clk_en",
                &format_args!("{}", self.usb_device_clk_en().bit()),
            )
            .field(
                "usb_device_rst_en",
                &format_args!("{}", self.usb_device_rst_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USB_DEVICE_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable usb_device clock"]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_clk_en(&mut self) -> USB_DEVICE_CLK_EN_W<USB_DEVICE_CONF_SPEC> {
        USB_DEVICE_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 0 to reset usb_device module"]
    #[inline(always)]
    #[must_use]
    pub fn usb_device_rst_en(&mut self) -> USB_DEVICE_RST_EN_W<USB_DEVICE_CONF_SPEC> {
        USB_DEVICE_RST_EN_W::new(self, 1)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "USB_DEVICE configuration register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_device_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_device_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_DEVICE_CONF_SPEC;
impl crate::RegisterSpec for USB_DEVICE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_device_conf::R`](R) reader structure"]
impl crate::Readable for USB_DEVICE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_device_conf::W`](W) writer structure"]
impl crate::Writable for USB_DEVICE_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_DEVICE_CONF to value 0x01"]
impl crate::Resettable for USB_DEVICE_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x01;
}
