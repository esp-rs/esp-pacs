#[doc = "Register `SYS_HP_USB_DEVICE_CTRL` reader"]
pub type R = crate::R<SYS_HP_USB_DEVICE_CTRL_SPEC>;
#[doc = "Register `SYS_HP_USB_DEVICE_CTRL` writer"]
pub type W = crate::W<SYS_HP_USB_DEVICE_CTRL_SPEC>;
#[doc = "Field `SYS_USB_DEVICE_48M_CLK_EN` reader - "]
pub type SYS_USB_DEVICE_48M_CLK_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_DEVICE_48M_CLK_EN` writer - "]
pub type SYS_USB_DEVICE_48M_CLK_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SYS_USB_DEVICE_RST_EN` reader - "]
pub type SYS_USB_DEVICE_RST_EN_R = crate::BitReader;
#[doc = "Field `SYS_USB_DEVICE_RST_EN` writer - "]
pub type SYS_USB_DEVICE_RST_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_usb_device_48m_clk_en(&self) -> SYS_USB_DEVICE_48M_CLK_EN_R {
        SYS_USB_DEVICE_48M_CLK_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_usb_device_rst_en(&self) -> SYS_USB_DEVICE_RST_EN_R {
        SYS_USB_DEVICE_RST_EN_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_HP_USB_DEVICE_CTRL")
            .field(
                "sys_usb_device_48m_clk_en",
                &self.sys_usb_device_48m_clk_en(),
            )
            .field("sys_usb_device_rst_en", &self.sys_usb_device_rst_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30"]
    #[inline(always)]
    pub fn sys_usb_device_48m_clk_en(
        &mut self,
    ) -> SYS_USB_DEVICE_48M_CLK_EN_W<'_, SYS_HP_USB_DEVICE_CTRL_SPEC> {
        SYS_USB_DEVICE_48M_CLK_EN_W::new(self, 30)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn sys_usb_device_rst_en(
        &mut self,
    ) -> SYS_USB_DEVICE_RST_EN_W<'_, SYS_HP_USB_DEVICE_CTRL_SPEC> {
        SYS_USB_DEVICE_RST_EN_W::new(self, 31)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_hp_usb_device_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_hp_usb_device_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_HP_USB_DEVICE_CTRL_SPEC;
impl crate::RegisterSpec for SYS_HP_USB_DEVICE_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_hp_usb_device_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_HP_USB_DEVICE_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_hp_usb_device_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_HP_USB_DEVICE_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_HP_USB_DEVICE_CTRL to value 0x4000_0000"]
impl crate::Resettable for SYS_HP_USB_DEVICE_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x4000_0000;
}
