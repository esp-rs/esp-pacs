#[doc = "Register `HPCORE0_RESET_CAUSE` reader"]
pub type R = crate::R<HPCORE0_RESET_CAUSE_SPEC>;
#[doc = "Register `HPCORE0_RESET_CAUSE` writer"]
pub type W = crate::W<HPCORE0_RESET_CAUSE_SPEC>;
#[doc = "Field `HPCORE0_RESET_FLAG` reader - need_des"]
pub type HPCORE0_RESET_FLAG_R = crate::BitReader;
#[doc = "Field `HPCORE0_RESET_CAUSE` reader - 6'h1: POR reset 6'h3: digital system software reset 6'h5: PMU HP system power down reset 6'h7: HP system reset from HP watchdog0 6'h8: HP system reset from HP watchdog1 6'h9: HP system reset from LP watchdog 6'hb: HP core reset from HP watchdog 6'hc: HP core software reset 6'hd: HP core reset from LP watchdog 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: efuse crc error reset 6'h16: HP usb jtag chip reset 6'h17: HP usb uart chip reset 6'h18: HP jtag reset 6'h1a: HP core lockup"]
pub type HPCORE0_RESET_CAUSE_R = crate::FieldReader;
#[doc = "Field `CLR` writer - need_des"]
pub type CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_RESET_FLAG_CLR` writer - need_des"]
pub type HPCORE0_RESET_FLAG_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn hpcore0_reset_flag(&self) -> HPCORE0_RESET_FLAG_R {
        HPCORE0_RESET_FLAG_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:6 - 6'h1: POR reset 6'h3: digital system software reset 6'h5: PMU HP system power down reset 6'h7: HP system reset from HP watchdog0 6'h8: HP system reset from HP watchdog1 6'h9: HP system reset from LP watchdog 6'hb: HP core reset from HP watchdog 6'hc: HP core software reset 6'hd: HP core reset from LP watchdog 6'hf: brown out reset 6'h10: LP watchdog chip reset 6'h12: super watch dog reset 6'h13: glitch reset 6'h14: efuse crc error reset 6'h16: HP usb jtag chip reset 6'h17: HP usb uart chip reset 6'h18: HP jtag reset 6'h1a: HP core lockup"]
    #[inline(always)]
    pub fn hpcore0_reset_cause(&self) -> HPCORE0_RESET_CAUSE_R {
        HPCORE0_RESET_CAUSE_R::new(((self.bits >> 1) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPCORE0_RESET_CAUSE")
            .field("hpcore0_reset_flag", &self.hpcore0_reset_flag())
            .field("hpcore0_reset_cause", &self.hpcore0_reset_cause())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn clr(&mut self) -> CLR_W<'_, HPCORE0_RESET_CAUSE_SPEC> {
        CLR_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn hpcore0_reset_flag_clr(
        &mut self,
    ) -> HPCORE0_RESET_FLAG_CLR_W<'_, HPCORE0_RESET_CAUSE_SPEC> {
        HPCORE0_RESET_FLAG_CLR_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hpcore0_reset_cause::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpcore0_reset_cause::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPCORE0_RESET_CAUSE_SPEC;
impl crate::RegisterSpec for HPCORE0_RESET_CAUSE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpcore0_reset_cause::R`](R) reader structure"]
impl crate::Readable for HPCORE0_RESET_CAUSE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpcore0_reset_cause::W`](W) writer structure"]
impl crate::Writable for HPCORE0_RESET_CAUSE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HPCORE0_RESET_CAUSE to value 0"]
impl crate::Resettable for HPCORE0_RESET_CAUSE_SPEC {}
