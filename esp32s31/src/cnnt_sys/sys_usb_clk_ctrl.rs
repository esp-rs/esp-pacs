#[doc = "Register `SYS_USB_CLK_CTRL` reader"]
pub type R = crate::R<SYS_USB_CLK_CTRL_SPEC>;
#[doc = "Register `SYS_USB_CLK_CTRL` writer"]
pub type W = crate::W<SYS_USB_CLK_CTRL_SPEC>;
#[doc = "Field `SYS_USB_48M_DIV_NUM` reader - "]
pub type SYS_USB_48M_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_USB_48M_DIV_NUM` writer - "]
pub type SYS_USB_48M_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYS_USB_25M_DIV_NUM` reader - "]
pub type SYS_USB_25M_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_USB_25M_DIV_NUM` writer - "]
pub type SYS_USB_25M_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SYS_USB_12M_DIV_NUM` reader - "]
pub type SYS_USB_12M_DIV_NUM_R = crate::FieldReader;
#[doc = "Field `SYS_USB_12M_DIV_NUM` writer - "]
pub type SYS_USB_12M_DIV_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sys_usb_48m_div_num(&self) -> SYS_USB_48M_DIV_NUM_R {
        SYS_USB_48M_DIV_NUM_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sys_usb_25m_div_num(&self) -> SYS_USB_25M_DIV_NUM_R {
        SYS_USB_25M_DIV_NUM_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sys_usb_12m_div_num(&self) -> SYS_USB_12M_DIV_NUM_R {
        SYS_USB_12M_DIV_NUM_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SYS_USB_CLK_CTRL")
            .field("sys_usb_48m_div_num", &self.sys_usb_48m_div_num())
            .field("sys_usb_25m_div_num", &self.sys_usb_25m_div_num())
            .field("sys_usb_12m_div_num", &self.sys_usb_12m_div_num())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sys_usb_48m_div_num(&mut self) -> SYS_USB_48M_DIV_NUM_W<'_, SYS_USB_CLK_CTRL_SPEC> {
        SYS_USB_48M_DIV_NUM_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn sys_usb_25m_div_num(&mut self) -> SYS_USB_25M_DIV_NUM_W<'_, SYS_USB_CLK_CTRL_SPEC> {
        SYS_USB_25M_DIV_NUM_W::new(self, 16)
    }
    #[doc = "Bits 24:31"]
    #[inline(always)]
    pub fn sys_usb_12m_div_num(&mut self) -> SYS_USB_12M_DIV_NUM_W<'_, SYS_USB_CLK_CTRL_SPEC> {
        SYS_USB_12M_DIV_NUM_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`sys_usb_clk_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sys_usb_clk_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SYS_USB_CLK_CTRL_SPEC;
impl crate::RegisterSpec for SYS_USB_CLK_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sys_usb_clk_ctrl::R`](R) reader structure"]
impl crate::Readable for SYS_USB_CLK_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sys_usb_clk_ctrl::W`](W) writer structure"]
impl crate::Writable for SYS_USB_CLK_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SYS_USB_CLK_CTRL to value 0x2713_0900"]
impl crate::Resettable for SYS_USB_CLK_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x2713_0900;
}
