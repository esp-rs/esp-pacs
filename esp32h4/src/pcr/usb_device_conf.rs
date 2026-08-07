#[doc = "Register `USB_DEVICE_CONF` reader"]
pub type R = crate::R<USB_DEVICE_CONF_SPEC>;
#[doc = "Register `USB_DEVICE_CONF` writer"]
pub type W = crate::W<USB_DEVICE_CONF_SPEC>;
#[doc = "Field `USB_DEVICE_CLK_EN` reader - Set 1 to enable usb_device clock"]
pub type USB_DEVICE_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_CLK_EN` writer - Set 1 to enable usb_device clock"]
pub type USB_DEVICE_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DEVICE_RST_EN` reader - Set 1 to reset usb_device module"]
pub type USB_DEVICE_RST_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_RST_EN` writer - Set 1 to reset usb_device module"]
pub type USB_DEVICE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DEVICE_READY` reader - Query this field after reset usb_device module"]
pub type USB_DEVICE_READY_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Set 1 to enable usb_device clock"]
    #[inline(always)]
    pub fn usb_device_clk_en(&self) -> USB_DEVICE_CLK_EN_R {
        USB_DEVICE_CLK_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set 1 to reset usb_device module"]
    #[inline(always)]
    pub fn usb_device_rst_en(&self) -> USB_DEVICE_RST_EN_R {
        USB_DEVICE_RST_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Query this field after reset usb_device module"]
    #[inline(always)]
    pub fn usb_device_ready(&self) -> USB_DEVICE_READY_R {
        USB_DEVICE_READY_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DEVICE_CONF")
            .field("usb_device_clk_en", &self.usb_device_clk_en())
            .field("usb_device_rst_en", &self.usb_device_rst_en())
            .field("usb_device_ready", &self.usb_device_ready())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set 1 to enable usb_device clock"]
    #[inline(always)]
    pub fn usb_device_clk_en(&mut self) -> USB_DEVICE_CLK_EN_W<'_, USB_DEVICE_CONF_SPEC> {
        USB_DEVICE_CLK_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set 1 to reset usb_device module"]
    #[inline(always)]
    pub fn usb_device_rst_en(&mut self) -> USB_DEVICE_RST_EN_W<'_, USB_DEVICE_CONF_SPEC> {
        USB_DEVICE_RST_EN_W::new(self, 1)
    }
}
#[doc = "USB_DEVICE configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_DEVICE_CONF_SPEC;
impl crate::RegisterSpec for USB_DEVICE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_device_conf::R`](R) reader structure"]
impl crate::Readable for USB_DEVICE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_device_conf::W`](W) writer structure"]
impl crate::Writable for USB_DEVICE_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_DEVICE_CONF to value 0x05"]
impl crate::Resettable for USB_DEVICE_CONF_SPEC {
    const RESET_VALUE: u32 = 0x05;
}
