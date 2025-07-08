#[doc = "Register `FIFO_CFG` reader"]
pub type R = crate::R<FIFO_CFG_SPEC>;
#[doc = "Register `FIFO_CFG` writer"]
pub type W = crate::W<FIFO_CFG_SPEC>;
#[doc = "Field `TX_FIFO_SRST` reader - Write 1 to reset async fifo in tx module."]
pub type TX_FIFO_SRST_R = crate::BitReader;
#[doc = "Field `TX_FIFO_SRST` writer - Write 1 to reset async fifo in tx module."]
pub type TX_FIFO_SRST_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX_FIFO_SRST` reader - Write 1 to reset async fifo in rx module."]
pub type RX_FIFO_SRST_R = crate::BitReader;
#[doc = "Field `RX_FIFO_SRST` writer - Write 1 to reset async fifo in rx module."]
pub type RX_FIFO_SRST_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 30 - Write 1 to reset async fifo in tx module."]
    #[inline(always)]
    pub fn tx_fifo_srst(&self) -> TX_FIFO_SRST_R {
        TX_FIFO_SRST_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - Write 1 to reset async fifo in rx module."]
    #[inline(always)]
    pub fn rx_fifo_srst(&self) -> RX_FIFO_SRST_R {
        RX_FIFO_SRST_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FIFO_CFG")
            .field("tx_fifo_srst", &self.tx_fifo_srst())
            .field("rx_fifo_srst", &self.rx_fifo_srst())
            .finish()
    }
}
impl W {
    #[doc = "Bit 30 - Write 1 to reset async fifo in tx module."]
    #[inline(always)]
    pub fn tx_fifo_srst(&mut self) -> TX_FIFO_SRST_W<FIFO_CFG_SPEC> {
        TX_FIFO_SRST_W::new(self, 30)
    }
    #[doc = "Bit 31 - Write 1 to reset async fifo in rx module."]
    #[inline(always)]
    pub fn rx_fifo_srst(&mut self) -> RX_FIFO_SRST_W<FIFO_CFG_SPEC> {
        RX_FIFO_SRST_W::new(self, 31)
    }
}
#[doc = "Parallel IO FIFO configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`fifo_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`fifo_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FIFO_CFG_SPEC;
impl crate::RegisterSpec for FIFO_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`fifo_cfg::R`](R) reader structure"]
impl crate::Readable for FIFO_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`fifo_cfg::W`](W) writer structure"]
impl crate::Writable for FIFO_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FIFO_CFG to value 0"]
impl crate::Resettable for FIFO_CFG_SPEC {}
