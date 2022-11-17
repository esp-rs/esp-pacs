#[doc = "Register `CONFIG_UPDATE` writer"]
pub struct W(crate::W<CONFIG_UPDATE_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<CONFIG_UPDATE_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<CONFIG_UPDATE_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<CONFIG_UPDATE_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `USB_SERIAL_JTAG_CONFIG_UPDATE` writer - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
pub type USB_SERIAL_JTAG_CONFIG_UPDATE_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, CONFIG_UPDATE_SPEC, bool, O>;
impl W {
    #[doc = "Bit 0 - Write 1 to this register would update the value of configure registers from APB clock domain to 48MHz clock domain."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_config_update(&mut self) -> USB_SERIAL_JTAG_CONFIG_UPDATE_W<0> {
        USB_SERIAL_JTAG_CONFIG_UPDATE_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "Configuration registers' value update\n\nThis register you can [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [config_update](index.html) module"]
pub struct CONFIG_UPDATE_SPEC;
impl crate::RegisterSpec for CONFIG_UPDATE_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [config_update::W](W) writer structure"]
impl crate::Writable for CONFIG_UPDATE_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets CONFIG_UPDATE to value 0"]
impl crate::Resettable for CONFIG_UPDATE_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
