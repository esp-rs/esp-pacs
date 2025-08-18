#[doc = "Register `TX_DSCR_CONF` reader"]
pub type R = crate::R<TX_DSCR_CONF_SPEC>;
#[doc = "Register `TX_DSCR_CONF` writer"]
pub type W = crate::W<TX_DSCR_CONF_SPEC>;
#[doc = "Field `WR_RETRY_THRESHOLD` reader - "]
pub type WR_RETRY_THRESHOLD_R = crate::FieldReader<u16>;
#[doc = "Field `WR_RETRY_THRESHOLD` writer - "]
pub type WR_RETRY_THRESHOLD_W<'a, REG> = crate::FieldWriter<'a, REG, 11, u16>;
impl R {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn wr_retry_threshold(&self) -> WR_RETRY_THRESHOLD_R {
        WR_RETRY_THRESHOLD_R::new((self.bits & 0x07ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TX_DSCR_CONF")
            .field("wr_retry_threshold", &self.wr_retry_threshold())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:10"]
    #[inline(always)]
    pub fn wr_retry_threshold(&mut self) -> WR_RETRY_THRESHOLD_W<'_, TX_DSCR_CONF_SPEC> {
        WR_RETRY_THRESHOLD_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`tx_dscr_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tx_dscr_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TX_DSCR_CONF_SPEC;
impl crate::RegisterSpec for TX_DSCR_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tx_dscr_conf::R`](R) reader structure"]
impl crate::Readable for TX_DSCR_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tx_dscr_conf::W`](W) writer structure"]
impl crate::Writable for TX_DSCR_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TX_DSCR_CONF to value 0x80"]
impl crate::Resettable for TX_DSCR_CONF_SPEC {
    const RESET_VALUE: u32 = 0x80;
}
