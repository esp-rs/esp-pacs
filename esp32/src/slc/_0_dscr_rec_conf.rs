#[doc = "Register `_0_DSCR_REC_CONF` reader"]
pub type R = crate::R<_0_DSCR_REC_CONF_SPEC>;
#[doc = "Register `_0_DSCR_REC_CONF` writer"]
pub type W = crate::W<_0_DSCR_REC_CONF_SPEC>;
#[doc = "Field `SLC0_RX_DSCR_REC_LIM` reader - "]
pub type SLC0_RX_DSCR_REC_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `SLC0_RX_DSCR_REC_LIM` writer - "]
pub type SLC0_RX_DSCR_REC_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc0_rx_dscr_rec_lim(&self) -> SLC0_RX_DSCR_REC_LIM_R {
        SLC0_RX_DSCR_REC_LIM_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("_0_DSCR_REC_CONF")
            .field("slc0_rx_dscr_rec_lim", &self.slc0_rx_dscr_rec_lim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9"]
    #[inline(always)]
    pub fn slc0_rx_dscr_rec_lim(&mut self) -> SLC0_RX_DSCR_REC_LIM_W<'_, _0_DSCR_REC_CONF_SPEC> {
        SLC0_RX_DSCR_REC_LIM_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`_0_dscr_rec_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`_0_dscr_rec_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct _0_DSCR_REC_CONF_SPEC;
impl crate::RegisterSpec for _0_DSCR_REC_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`_0_dscr_rec_conf::R`](R) reader structure"]
impl crate::Readable for _0_DSCR_REC_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`_0_dscr_rec_conf::W`](W) writer structure"]
impl crate::Writable for _0_DSCR_REC_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets _0_DSCR_REC_CONF to value 0x03ff"]
impl crate::Resettable for _0_DSCR_REC_CONF_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
