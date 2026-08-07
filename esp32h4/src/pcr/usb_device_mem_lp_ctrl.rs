#[doc = "Register `USB_DEVICE_MEM_LP_CTRL` reader"]
pub type R = crate::R<USB_DEVICE_MEM_LP_CTRL_SPEC>;
#[doc = "Register `USB_DEVICE_MEM_LP_CTRL` writer"]
pub type W = crate::W<USB_DEVICE_MEM_LP_CTRL_SPEC>;
#[doc = "Field `USB_DEVICE_MEM_LP_MODE` reader - Configures usb_device memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type USB_DEVICE_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `USB_DEVICE_MEM_LP_MODE` writer - Configures usb_device memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
pub type USB_DEVICE_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `USB_DEVICE_MEM_LP_EN` reader - Set this bit to power down usb_device memory."]
pub type USB_DEVICE_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_MEM_LP_EN` writer - Set this bit to power down usb_device memory."]
pub type USB_DEVICE_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_DEVICE_MEM_FORCE_CTRL` reader - Set this bit to force software control usb_device memory, disbale hardware control."]
pub type USB_DEVICE_MEM_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `USB_DEVICE_MEM_FORCE_CTRL` writer - Set this bit to force software control usb_device memory, disbale hardware control."]
pub type USB_DEVICE_MEM_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - Configures usb_device memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn usb_device_mem_lp_mode(&self) -> USB_DEVICE_MEM_LP_MODE_R {
        USB_DEVICE_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - Set this bit to power down usb_device memory."]
    #[inline(always)]
    pub fn usb_device_mem_lp_en(&self) -> USB_DEVICE_MEM_LP_EN_R {
        USB_DEVICE_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to force software control usb_device memory, disbale hardware control."]
    #[inline(always)]
    pub fn usb_device_mem_force_ctrl(&self) -> USB_DEVICE_MEM_FORCE_CTRL_R {
        USB_DEVICE_MEM_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_DEVICE_MEM_LP_CTRL")
            .field("usb_device_mem_lp_mode", &self.usb_device_mem_lp_mode())
            .field("usb_device_mem_lp_en", &self.usb_device_mem_lp_en())
            .field(
                "usb_device_mem_force_ctrl",
                &self.usb_device_mem_force_ctrl(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - Configures usb_device memory low power mode in low power stage.\\\\ 0(default): deep sleep\\\\ 1: light sleep\\\\ 2: shut down\\\\ 3: disable low power stage\\\\"]
    #[inline(always)]
    pub fn usb_device_mem_lp_mode(
        &mut self,
    ) -> USB_DEVICE_MEM_LP_MODE_W<'_, USB_DEVICE_MEM_LP_CTRL_SPEC> {
        USB_DEVICE_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2 - Set this bit to power down usb_device memory."]
    #[inline(always)]
    pub fn usb_device_mem_lp_en(
        &mut self,
    ) -> USB_DEVICE_MEM_LP_EN_W<'_, USB_DEVICE_MEM_LP_CTRL_SPEC> {
        USB_DEVICE_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to force software control usb_device memory, disbale hardware control."]
    #[inline(always)]
    pub fn usb_device_mem_force_ctrl(
        &mut self,
    ) -> USB_DEVICE_MEM_FORCE_CTRL_W<'_, USB_DEVICE_MEM_LP_CTRL_SPEC> {
        USB_DEVICE_MEM_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "USB_DEVICE memory power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_device_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_device_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_DEVICE_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for USB_DEVICE_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_device_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for USB_DEVICE_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_device_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for USB_DEVICE_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_DEVICE_MEM_LP_CTRL to value 0"]
impl crate::Resettable for USB_DEVICE_MEM_LP_CTRL_SPEC {}
