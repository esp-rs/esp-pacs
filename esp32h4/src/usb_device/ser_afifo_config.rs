#[doc = "Register `SER_AFIFO_CONFIG` reader"]
pub type R = crate::R<SER_AFIFO_CONFIG_SPEC>;
#[doc = "Register `SER_AFIFO_CONFIG` writer"]
pub type W = crate::W<SER_AFIFO_CONFIG_SPEC>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR` reader - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR` writer - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD` reader - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD` writer - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR` reader - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR` writer - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD` reader - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD` writer - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY` reader - CDC_ACM OUTOUT async FIFO empty signal in read clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_R = crate::BitReader;
#[doc = "Field `USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL` reader - CDC_ACM OUT IN async FIFO empty signal in write clock domain."]
pub type USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_reset_wr(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_R {
        USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_reset_rd(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_R {
        USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_reset_wr(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_R {
        USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_reset_rd(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_R {
        USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - CDC_ACM OUTOUT async FIFO empty signal in read clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_rempty(
        &self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_R {
        USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_REMPTY_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - CDC_ACM OUT IN async FIFO empty signal in write clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_wfull(&self) -> USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_R {
        USB_SERIAL_JTAG_SERIAL_IN_AFIFO_WFULL_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SER_AFIFO_CONFIG")
            .field(
                "usb_serial_jtag_serial_in_afifo_reset_wr",
                &self.usb_serial_jtag_serial_in_afifo_reset_wr(),
            )
            .field(
                "usb_serial_jtag_serial_in_afifo_reset_rd",
                &self.usb_serial_jtag_serial_in_afifo_reset_rd(),
            )
            .field(
                "usb_serial_jtag_serial_out_afifo_reset_wr",
                &self.usb_serial_jtag_serial_out_afifo_reset_wr(),
            )
            .field(
                "usb_serial_jtag_serial_out_afifo_reset_rd",
                &self.usb_serial_jtag_serial_out_afifo_reset_rd(),
            )
            .field(
                "usb_serial_jtag_serial_out_afifo_rempty",
                &self.usb_serial_jtag_serial_out_afifo_rempty(),
            )
            .field(
                "usb_serial_jtag_serial_in_afifo_wfull",
                &self.usb_serial_jtag_serial_in_afifo_wfull(),
            )
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Write 1 to reset CDC_ACM IN async FIFO write clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_reset_wr(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_W<'_, SER_AFIFO_CONFIG_SPEC> {
        USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_WR_W::new(self, 0)
    }
    #[doc = "Bit 1 - Write 1 to reset CDC_ACM IN async FIFO read clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_in_afifo_reset_rd(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_W<'_, SER_AFIFO_CONFIG_SPEC> {
        USB_SERIAL_JTAG_SERIAL_IN_AFIFO_RESET_RD_W::new(self, 1)
    }
    #[doc = "Bit 2 - Write 1 to reset CDC_ACM OUT async FIFO write clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_reset_wr(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_W<'_, SER_AFIFO_CONFIG_SPEC> {
        USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_WR_W::new(self, 2)
    }
    #[doc = "Bit 3 - Write 1 to reset CDC_ACM OUT async FIFO read clock domain."]
    #[inline(always)]
    pub fn usb_serial_jtag_serial_out_afifo_reset_rd(
        &mut self,
    ) -> USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_W<'_, SER_AFIFO_CONFIG_SPEC> {
        USB_SERIAL_JTAG_SERIAL_OUT_AFIFO_RESET_RD_W::new(self, 3)
    }
}
#[doc = "Serial AFIFO configure register\n\nYou can [`read`](crate::Reg::read) this register and get [`ser_afifo_config::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ser_afifo_config::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SER_AFIFO_CONFIG_SPEC;
impl crate::RegisterSpec for SER_AFIFO_CONFIG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ser_afifo_config::R`](R) reader structure"]
impl crate::Readable for SER_AFIFO_CONFIG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ser_afifo_config::W`](W) writer structure"]
impl crate::Writable for SER_AFIFO_CONFIG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SER_AFIFO_CONFIG to value 0x10"]
impl crate::Resettable for SER_AFIFO_CONFIG_SPEC {
    const RESET_VALUE: u32 = 0x10;
}
