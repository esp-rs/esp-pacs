#[doc = "Register `OUT_LINK` reader"]
pub type R = crate::R<OUT_LINK_SPEC>;
#[doc = "Register `OUT_LINK` writer"]
pub type W = crate::W<OUT_LINK_SPEC>;
#[doc = "Field `OUTLINK_ADDR` reader - The address of first outlink descriptor."]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - The address of first outlink descriptor."]
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `OUTLINK_STOP` reader - Set this bit to stop outlink descriptor."]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - Set this bit to stop outlink descriptor."]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` reader - Set this bit to start outlink descriptor."]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - Set this bit to start outlink descriptor."]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` reader - Set this bit to restart outlink descriptor."]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - Set this bit to restart outlink descriptor."]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK` reader - "]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19 - The address of first outlink descriptor."]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28 - Set this bit to stop outlink descriptor."]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - Set this bit to start outlink descriptor."]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - Set this bit to restart outlink descriptor."]
    #[inline(always)]
    pub fn outlink_restart(&self) -> OUTLINK_RESTART_R {
        OUTLINK_RESTART_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn outlink_park(&self) -> OUTLINK_PARK_R {
        OUTLINK_PARK_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_LINK")
            .field("outlink_addr", &self.outlink_addr())
            .field("outlink_stop", &self.outlink_stop())
            .field("outlink_start", &self.outlink_start())
            .field("outlink_restart", &self.outlink_restart())
            .field("outlink_park", &self.outlink_park())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:19 - The address of first outlink descriptor."]
    #[inline(always)]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<OUT_LINK_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28 - Set this bit to stop outlink descriptor."]
    #[inline(always)]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<OUT_LINK_SPEC> {
        OUTLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29 - Set this bit to start outlink descriptor."]
    #[inline(always)]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<OUT_LINK_SPEC> {
        OUTLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30 - Set this bit to restart outlink descriptor."]
    #[inline(always)]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<OUT_LINK_SPEC> {
        OUTLINK_RESTART_W::new(self, 30)
    }
}
#[doc = "I2S DMA TX configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`out_link::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_link::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_LINK_SPEC;
impl crate::RegisterSpec for OUT_LINK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_link::R`](R) reader structure"]
impl crate::Readable for OUT_LINK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_link::W`](W) writer structure"]
impl crate::Writable for OUT_LINK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_LINK to value 0"]
impl crate::Resettable for OUT_LINK_SPEC {
    const RESET_VALUE: u32 = 0;
}
