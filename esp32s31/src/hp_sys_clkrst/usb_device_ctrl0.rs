#[doc = "Register `USB_DEVICE_CTRL0` reader"]
pub type R = crate::R<USB_DEVICE_CTRL0_SPEC>;
#[doc = "Register `USB_DEVICE_CTRL0` writer"]
pub type W = crate::W<USB_DEVICE_CTRL0_SPEC>;
#[doc = "Field `REG_USB_DEVICE_APB_CLK_EN` reader - need_des"]
pub type REG_USB_DEVICE_APB_CLK_EN_R = crate::BitReader;
#[doc = "Field `REG_USB_DEVICE_APB_CLK_EN` writer - need_des"]
pub type REG_USB_DEVICE_APB_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_usb_device_apb_clk_en(&self) -> REG_USB_DEVICE_APB_CLK_EN_R {
        REG_USB_DEVICE_APB_CLK_EN_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DEVICE_CTRL0")
            .field(
                "reg_usb_device_apb_clk_en",
                &self.reg_usb_device_apb_clk_en(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn reg_usb_device_apb_clk_en(
        &mut self,
    ) -> REG_USB_DEVICE_APB_CLK_EN_W<'_, USB_DEVICE_CTRL0_SPEC> {
        REG_USB_DEVICE_APB_CLK_EN_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_DEVICE_CTRL0_SPEC;
impl crate::RegisterSpec for USB_DEVICE_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_device_ctrl0::R`](R) reader structure"]
impl crate::Readable for USB_DEVICE_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_device_ctrl0::W`](W) writer structure"]
impl crate::Writable for USB_DEVICE_CTRL0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_DEVICE_CTRL0 to value 0x01"]
impl crate::Resettable for USB_DEVICE_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x01;
}
