#[doc = "Register `USB_SERIAL_JTAG_JFIFO_ST` reader"]
pub type R = crate::R<USB_SERIAL_JTAG_JFIFO_ST_SPEC>;
#[doc = "Register `USB_SERIAL_JTAG_JFIFO_ST` writer"]
pub type W = crate::W<USB_SERIAL_JTAG_JFIFO_ST_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_IN_FIFO_CNT` reader - JTAT in fifo counter."]
pub type USB_SERIAL_JTAG_IN_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_FIFO_EMPTY` reader - 1: JTAG in fifo is empty."]
pub type USB_SERIAL_JTAG_IN_FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_FIFO_FULL` reader - 1: JTAG in fifo is full."]
pub type USB_SERIAL_JTAG_IN_FIFO_FULL_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_FIFO_CNT` reader - JTAT out fifo counter."]
pub type USB_SERIAL_JTAG_OUT_FIFO_CNT_R = crate::FieldReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_FIFO_EMPTY` reader - 1: JTAG out fifo is empty."]
pub type USB_SERIAL_JTAG_OUT_FIFO_EMPTY_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_FIFO_FULL` reader - 1: JTAG out fifo is full."]
pub type USB_SERIAL_JTAG_OUT_FIFO_FULL_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_FIFO_RESET` reader - Write 1 to reset JTAG in fifo."]
pub type USB_SERIAL_JTAG_IN_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_IN_FIFO_RESET` writer - Write 1 to reset JTAG in fifo."]
pub type USB_SERIAL_JTAG_IN_FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_OUT_FIFO_RESET` reader - Write 1 to reset JTAG out fifo."]
pub type USB_SERIAL_JTAG_OUT_FIFO_RESET_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_OUT_FIFO_RESET` writer - Write 1 to reset JTAG out fifo."]
pub type USB_SERIAL_JTAG_OUT_FIFO_RESET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1 - JTAT in fifo counter."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_cnt(&self) -> USB_SERIAL_JTAG_IN_FIFO_CNT_R {
        USB_SERIAL_JTAG_IN_FIFO_CNT_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2 - 1: JTAG in fifo is empty."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_empty(&self) -> USB_SERIAL_JTAG_IN_FIFO_EMPTY_R {
        USB_SERIAL_JTAG_IN_FIFO_EMPTY_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - 1: JTAG in fifo is full."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_full(&self) -> USB_SERIAL_JTAG_IN_FIFO_FULL_R {
        USB_SERIAL_JTAG_IN_FIFO_FULL_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:5 - JTAT out fifo counter."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_cnt(&self) -> USB_SERIAL_JTAG_OUT_FIFO_CNT_R {
        USB_SERIAL_JTAG_OUT_FIFO_CNT_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bit 6 - 1: JTAG out fifo is empty."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_empty(&self) -> USB_SERIAL_JTAG_OUT_FIFO_EMPTY_R {
        USB_SERIAL_JTAG_OUT_FIFO_EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - 1: JTAG out fifo is full."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_full(&self) -> USB_SERIAL_JTAG_OUT_FIFO_FULL_R {
        USB_SERIAL_JTAG_OUT_FIFO_FULL_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Write 1 to reset JTAG in fifo."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_reset(&self) -> USB_SERIAL_JTAG_IN_FIFO_RESET_R {
        USB_SERIAL_JTAG_IN_FIFO_RESET_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Write 1 to reset JTAG out fifo."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_reset(&self) -> USB_SERIAL_JTAG_OUT_FIFO_RESET_R {
        USB_SERIAL_JTAG_OUT_FIFO_RESET_R::new(((self.bits >> 9) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("USB_SERIAL_JTAG_JFIFO_ST")
            .field(
                "usb_serial_jtag_in_fifo_cnt",
                &self.usb_serial_jtag_in_fifo_cnt(),
            )
            .field(
                "usb_serial_jtag_in_fifo_empty",
                &self.usb_serial_jtag_in_fifo_empty(),
            )
            .field(
                "usb_serial_jtag_in_fifo_full",
                &self.usb_serial_jtag_in_fifo_full(),
            )
            .field(
                "usb_serial_jtag_out_fifo_cnt",
                &self.usb_serial_jtag_out_fifo_cnt(),
            )
            .field(
                "usb_serial_jtag_out_fifo_empty",
                &self.usb_serial_jtag_out_fifo_empty(),
            )
            .field(
                "usb_serial_jtag_out_fifo_full",
                &self.usb_serial_jtag_out_fifo_full(),
            )
            .field(
                "usb_serial_jtag_in_fifo_reset",
                &self.usb_serial_jtag_in_fifo_reset(),
            )
            .field(
                "usb_serial_jtag_out_fifo_reset",
                &self.usb_serial_jtag_out_fifo_reset(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 8 - Write 1 to reset JTAG in fifo."]
    #[inline(always)]
    pub fn usb_serial_jtag_in_fifo_reset(
        &mut self,
    ) -> USB_SERIAL_JTAG_IN_FIFO_RESET_W<'_, USB_SERIAL_JTAG_JFIFO_ST_SPEC> {
        USB_SERIAL_JTAG_IN_FIFO_RESET_W::new(self, 8)
    }
    #[doc = "Bit 9 - Write 1 to reset JTAG out fifo."]
    #[inline(always)]
    pub fn usb_serial_jtag_out_fifo_reset(
        &mut self,
    ) -> USB_SERIAL_JTAG_OUT_FIFO_RESET_W<'_, USB_SERIAL_JTAG_JFIFO_ST_SPEC> {
        USB_SERIAL_JTAG_OUT_FIFO_RESET_W::new(self, 9)
    }
}
#[doc = "JTAG FIFO status and control registers.\n\nYou can [`read`](crate::Reg::read) this register and get [`usb_serial_jtag_jfifo_st::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`usb_serial_jtag_jfifo_st::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct USB_SERIAL_JTAG_JFIFO_ST_SPEC;
impl crate::RegisterSpec for USB_SERIAL_JTAG_JFIFO_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`usb_serial_jtag_jfifo_st::R`](R) reader structure"]
impl crate::Readable for USB_SERIAL_JTAG_JFIFO_ST_SPEC {}
#[doc = "`write(|w| ..)` method takes [`usb_serial_jtag_jfifo_st::W`](W) writer structure"]
impl crate::Writable for USB_SERIAL_JTAG_JFIFO_ST_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets USB_SERIAL_JTAG_JFIFO_ST to value 0x44"]
impl crate::Resettable for USB_SERIAL_JTAG_JFIFO_ST_SPEC {
    const RESET_VALUE: u32 = 0x44;
}
