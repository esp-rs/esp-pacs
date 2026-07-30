#[doc = "Register `USB_SERIAL_JTAG_GET_LINE_CODE_W1` reader"]
pub type R = crate::R<USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC>;
#[doc = "Register `USB_SERIAL_JTAG_GET_LINE_CODE_W1` writer"]
pub type W = crate::W<USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_GET_BDATA_BITS` reader - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_GET_BDATA_BITS_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_GET_BDATA_BITS` writer - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_GET_BDATA_BITS_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USB_SERIAL_JTAG_GET_BPARITY_TYPE` reader - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_GET_BPARITY_TYPE_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_GET_BPARITY_TYPE` writer - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_GET_BPARITY_TYPE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `USB_SERIAL_JTAG_GET_BCHAR_FORMAT` reader - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_GET_BCHAR_FORMAT_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_GET_BCHAR_FORMAT` writer - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
pub type USB_SERIAL_JTAG_GET_BCHAR_FORMAT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_bdata_bits(&self) -> USB_SERIAL_JTAG_GET_BDATA_BITS_R {
        USB_SERIAL_JTAG_GET_BDATA_BITS_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_bparity_type(&self) -> USB_SERIAL_JTAG_GET_BPARITY_TYPE_R {
        USB_SERIAL_JTAG_GET_BPARITY_TYPE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_bchar_format(&self) -> USB_SERIAL_JTAG_GET_BCHAR_FORMAT_R {
        USB_SERIAL_JTAG_GET_BCHAR_FORMAT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_SERIAL_JTAG_GET_LINE_CODE_W1")
            .field(
                "usb_serial_jtag_get_bdata_bits",
                &self.usb_serial_jtag_get_bdata_bits(),
            )
            .field(
                "usb_serial_jtag_get_bparity_type",
                &self.usb_serial_jtag_get_bparity_type(),
            )
            .field(
                "usb_serial_jtag_get_bchar_format",
                &self.usb_serial_jtag_get_bchar_format(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - The value of bCharFormat set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_bdata_bits(
        &mut self,
    ) -> USB_SERIAL_JTAG_GET_BDATA_BITS_W<'_, USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC> {
        USB_SERIAL_JTAG_GET_BDATA_BITS_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - The value of bParityTpye set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_bparity_type(
        &mut self,
    ) -> USB_SERIAL_JTAG_GET_BPARITY_TYPE_W<'_, USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC> {
        USB_SERIAL_JTAG_GET_BPARITY_TYPE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - The value of bDataBits set by software which is requested by GET_LINE_CODING command."]
    #[inline(always)]
    pub fn usb_serial_jtag_get_bchar_format(
        &mut self,
    ) -> USB_SERIAL_JTAG_GET_BCHAR_FORMAT_W<'_, USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC> {
        USB_SERIAL_JTAG_GET_BCHAR_FORMAT_W::new(self, 16)
    }
}
#[doc = "W1 of GET_LINE_CODING command.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_get_line_code_w1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_get_line_code_w1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC;
impl crate::RegisterSpec for USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_serial_jtag_get_line_code_w1::R`](R) reader structure"]
impl crate::Readable for USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_serial_jtag_get_line_code_w1::W`](W) writer structure"]
impl crate::Writable for USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_SERIAL_JTAG_GET_LINE_CODE_W1 to value 0"]
impl crate::Resettable for USB_SERIAL_JTAG_GET_LINE_CODE_W1_SPEC {}
