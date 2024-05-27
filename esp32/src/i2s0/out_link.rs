#[doc = "Register `OUT_LINK` reader"]
pub type R = crate::R<OUT_LINK_SPEC>;
#[doc = "Register `OUT_LINK` writer"]
pub type W = crate::W<OUT_LINK_SPEC>;
#[doc = "Field `OUTLINK_ADDR` reader - "]
pub type OUTLINK_ADDR_R = crate::FieldReader<u32>;
#[doc = "Field `OUTLINK_ADDR` writer - "]
pub type OUTLINK_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 20, u32>;
#[doc = "Field `OUTLINK_STOP` reader - "]
pub type OUTLINK_STOP_R = crate::BitReader;
#[doc = "Field `OUTLINK_STOP` writer - "]
pub type OUTLINK_STOP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_START` reader - "]
pub type OUTLINK_START_R = crate::BitReader;
#[doc = "Field `OUTLINK_START` writer - "]
pub type OUTLINK_START_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_RESTART` reader - "]
pub type OUTLINK_RESTART_R = crate::BitReader;
#[doc = "Field `OUTLINK_RESTART` writer - "]
pub type OUTLINK_RESTART_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUTLINK_PARK` reader - "]
pub type OUTLINK_PARK_R = crate::BitReader;
impl R {
    #[doc = "Bits 0:19"]
    #[inline(always)]
    pub fn outlink_addr(&self) -> OUTLINK_ADDR_R {
        OUTLINK_ADDR_R::new(self.bits & 0x000f_ffff)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    pub fn outlink_stop(&self) -> OUTLINK_STOP_R {
        OUTLINK_STOP_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    pub fn outlink_start(&self) -> OUTLINK_START_R {
        OUTLINK_START_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30"]
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
    #[doc = "Bits 0:19"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_addr(&mut self) -> OUTLINK_ADDR_W<OUT_LINK_SPEC> {
        OUTLINK_ADDR_W::new(self, 0)
    }
    #[doc = "Bit 28"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_stop(&mut self) -> OUTLINK_STOP_W<OUT_LINK_SPEC> {
        OUTLINK_STOP_W::new(self, 28)
    }
    #[doc = "Bit 29"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_start(&mut self) -> OUTLINK_START_W<OUT_LINK_SPEC> {
        OUTLINK_START_W::new(self, 29)
    }
    #[doc = "Bit 30"]
    #[inline(always)]
    #[must_use]
    pub fn outlink_restart(&mut self) -> OUTLINK_RESTART_W<OUT_LINK_SPEC> {
        OUTLINK_RESTART_W::new(self, 30)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_link::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_link::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
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
