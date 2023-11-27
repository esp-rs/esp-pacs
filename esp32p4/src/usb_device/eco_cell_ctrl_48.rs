#[doc = "Register `ECO_CELL_CTRL_48` reader"]
pub type R = crate::R<ECO_CELL_CTRL_48_SPEC>;
#[doc = "Register `ECO_CELL_CTRL_48` writer"]
pub type W = crate::W<ECO_CELL_CTRL_48_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_RDN_RESULT_48` reader - Reserved."]
pub type USB_SERIAL_JTAG_RDN_RESULT_48_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_RDN_ENA_48` reader - Reserved."]
pub type USB_SERIAL_JTAG_RDN_ENA_48_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_RDN_ENA_48` writer - Reserved."]
pub type USB_SERIAL_JTAG_RDN_ENA_48_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Reserved."]
    #[inline(always)]
    pub fn usb_serial_jtag_rdn_result_48(&self) -> USB_SERIAL_JTAG_RDN_RESULT_48_R {
        USB_SERIAL_JTAG_RDN_RESULT_48_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    pub fn usb_serial_jtag_rdn_ena_48(&self) -> USB_SERIAL_JTAG_RDN_ENA_48_R {
        USB_SERIAL_JTAG_RDN_ENA_48_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ECO_CELL_CTRL_48")
            .field(
                "usb_serial_jtag_rdn_result_48",
                &format_args!("{}", self.usb_serial_jtag_rdn_result_48().bit()),
            )
            .field(
                "usb_serial_jtag_rdn_ena_48",
                &format_args!("{}", self.usb_serial_jtag_rdn_ena_48().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<ECO_CELL_CTRL_48_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 1 - Reserved."]
    #[inline(always)]
    #[must_use]
    pub fn usb_serial_jtag_rdn_ena_48(
        &mut self,
    ) -> USB_SERIAL_JTAG_RDN_ENA_48_W<ECO_CELL_CTRL_48_SPEC> {
        USB_SERIAL_JTAG_RDN_ENA_48_W::new(self, 1)
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
#[doc = "Reserved.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`eco_cell_ctrl_48::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`eco_cell_ctrl_48::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ECO_CELL_CTRL_48_SPEC;
impl crate::RegisterSpec for ECO_CELL_CTRL_48_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`eco_cell_ctrl_48::R`](R) reader structure"]
impl crate::Readable for ECO_CELL_CTRL_48_SPEC {}
#[doc = "`write(|w| ..)` method takes [`eco_cell_ctrl_48::W`](W) writer structure"]
impl crate::Writable for ECO_CELL_CTRL_48_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets ECO_CELL_CTRL_48 to value 0"]
impl crate::Resettable for ECO_CELL_CTRL_48_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
