#[doc = "Register `USB_CONF` reader"]
pub type R = crate::R<USB_CONF_SPEC>;
#[doc = "Register `USB_CONF` writer"]
pub type W = crate::W<USB_CONF_SPEC>;
#[doc = "Field `IO_MUX_RESET_DISABLE` reader - Need add desc"]
pub type IO_MUX_RESET_DISABLE_R = crate::BitReader;
#[doc = "Field `IO_MUX_RESET_DISABLE` writer - Need add desc"]
pub type IO_MUX_RESET_DISABLE_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    pub fn io_mux_reset_disable(&self) -> IO_MUX_RESET_DISABLE_R {
        IO_MUX_RESET_DISABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_CONF")
            .field(
                "io_mux_reset_disable",
                &format_args!("{}", self.io_mux_reset_disable().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<USB_CONF_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 18 - Need add desc"]
    #[inline(always)]
    #[must_use]
    pub fn io_mux_reset_disable(&mut self) -> IO_MUX_RESET_DISABLE_W<USB_CONF_SPEC, 18> {
        IO_MUX_RESET_DISABLE_W::new(self)
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
#[doc = "register description\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`usb_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`usb_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_CONF_SPEC;
impl crate::RegisterSpec for USB_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_conf::R`](R) reader structure"]
impl crate::Readable for USB_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_conf::W`](W) writer structure"]
impl crate::Writable for USB_CONF_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets USB_CONF to value 0"]
impl crate::Resettable for USB_CONF_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
