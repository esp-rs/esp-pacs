#[doc = "Register `TX_START_CFG` reader"]
pub type R = crate::R<TX_START_CFG_SPEC>;
#[doc = "Register `TX_START_CFG` writer"]
pub type W = crate::W<TX_START_CFG_SPEC>;
#[doc = "Field `TX_START` reader - Set this bit to start tx data transmit."]
pub type TX_START_R = crate::BitReader;
#[doc = "Field `TX_START` writer - Set this bit to start tx data transmit."]
pub type TX_START_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - Set this bit to start tx data transmit."]
    #[inline(always)]
    pub fn tx_start(&self) -> TX_START_R {
        TX_START_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_START_CFG")
            .field("tx_start", &self.tx_start())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - Set this bit to start tx data transmit."]
    #[inline(always)]
    pub fn tx_start(&mut self) -> TX_START_W<'_, TX_START_CFG_SPEC> {
        TX_START_W::new(self, 31)
    }
}
#[doc = "Parallel TX Start configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_start_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_start_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_START_CFG_SPEC;
impl crate::RegisterSpec for TX_START_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_start_cfg::R`](R) reader structure"]
impl crate::Readable for TX_START_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_start_cfg::W`](W) writer structure"]
impl crate::Writable for TX_START_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_START_CFG to value 0"]
impl crate::Resettable for TX_START_CFG_SPEC {}
