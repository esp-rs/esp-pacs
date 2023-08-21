#[doc = "Register `MEM_CONF` reader"]
pub type R = crate::R<MEM_CONF_SPEC>;
#[doc = "Register `MEM_CONF` writer"]
pub type W = crate::W<MEM_CONF_SPEC>;
#[doc = "Field `USB_MEM_PD` reader - 1: power down usb memory."]
pub type USB_MEM_PD_R = crate::BitReader;
#[doc = "Field `USB_MEM_PD` writer - 1: power down usb memory."]
pub type USB_MEM_PD_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `USB_MEM_CLK_EN` reader - 1: Force clock on for usb memory."]
pub type USB_MEM_CLK_EN_R = crate::BitReader;
#[doc = "Field `USB_MEM_CLK_EN` writer - 1: Force clock on for usb memory."]
pub type USB_MEM_CLK_EN_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 0 - 1: power down usb memory."]
    #[inline(always)]
    pub fn usb_mem_pd(&self) -> USB_MEM_PD_R {
        USB_MEM_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - 1: Force clock on for usb memory."]
    #[inline(always)]
    pub fn usb_mem_clk_en(&self) -> USB_MEM_CLK_EN_R {
        USB_MEM_CLK_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CONF")
            .field("usb_mem_pd", &format_args!("{}", self.usb_mem_pd().bit()))
            .field(
                "usb_mem_clk_en",
                &format_args!("{}", self.usb_mem_clk_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 1: power down usb memory."]
    #[inline(always)]
    #[must_use]
    pub fn usb_mem_pd(&mut self) -> USB_MEM_PD_W<MEM_CONF_SPEC, 0> {
        USB_MEM_PD_W::new(self)
    }
    #[doc = "Bit 1 - 1: Force clock on for usb memory."]
    #[inline(always)]
    #[must_use]
    pub fn usb_mem_clk_en(&mut self) -> USB_MEM_CLK_EN_W<MEM_CONF_SPEC, 1> {
        USB_MEM_CLK_EN_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "Memory power control\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`mem_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`mem_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CONF_SPEC;
impl crate::RegisterSpec for MEM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_conf::R`](R) reader structure"]
impl crate::Readable for MEM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_conf::W`](W) writer structure"]
impl crate::Writable for MEM_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets MEM_CONF to value 0x02"]
impl crate::Resettable for MEM_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0x02;
}
