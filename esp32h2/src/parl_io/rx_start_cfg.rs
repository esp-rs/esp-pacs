#[doc = "Register `RX_START_CFG` reader"]
pub type R = crate::R<RX_START_CFG_SPEC>;
#[doc = "Register `RX_START_CFG` writer"]
pub type W = crate::W<RX_START_CFG_SPEC>;
#[doc = "Field `RX_START` reader - Set this bit to start rx data sampling."]
pub type RX_START_R = crate::BitReader;
#[doc = "Field `RX_START` writer - Set this bit to start rx data sampling."]
pub type RX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Set this bit to start rx data sampling."]
    #[inline(always)]
    pub fn rx_start(&self) -> RX_START_R {
        RX_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RX_START_CFG")
            .field("rx_start", &self.rx_start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to start rx data sampling."]
    #[inline(always)]
    pub fn rx_start(&mut self) -> RX_START_W<RX_START_CFG_SPEC> {
        RX_START_W::new(self, 31)
    }
}
#[doc = "Parallel RX Start configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`rx_start_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rx_start_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RX_START_CFG_SPEC;
impl crate::RegisterSpec for RX_START_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rx_start_cfg::R`](R) reader structure"]
impl crate::Readable for RX_START_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rx_start_cfg::W`](W) writer structure"]
impl crate::Writable for RX_START_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RX_START_CFG to value 0"]
impl crate::Resettable for RX_START_CFG_SPEC {}
