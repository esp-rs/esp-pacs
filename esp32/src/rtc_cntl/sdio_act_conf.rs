#[doc = "Register `SDIO_ACT_CONF` reader"]
pub type R = crate::R<SDIO_ACT_CONF_SPEC>;
#[doc = "Register `SDIO_ACT_CONF` writer"]
pub type W = crate::W<SDIO_ACT_CONF_SPEC>;
#[doc = "Field `SDIO_ACT_DNUM` reader - "]
pub type SDIO_ACT_DNUM_R = crate::FieldReader<u16>;
#[doc = "Field `SDIO_ACT_DNUM` writer - "]
pub type SDIO_ACT_DNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    pub fn sdio_act_dnum(&self) -> SDIO_ACT_DNUM_R {
        SDIO_ACT_DNUM_R::new(((self.bits >> 22) & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_ACT_CONF")
            .field("sdio_act_dnum", &self.sdio_act_dnum())
            .finish()
    }
}
impl W {
    #[doc = "Bits 22:31"]
    #[inline(always)]
    #[must_use]
    pub fn sdio_act_dnum(&mut self) -> SDIO_ACT_DNUM_W<SDIO_ACT_CONF_SPEC> {
        SDIO_ACT_DNUM_W::new(self, 22)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`sdio_act_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`sdio_act_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_ACT_CONF_SPEC;
impl crate::RegisterSpec for SDIO_ACT_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_act_conf::R`](R) reader structure"]
impl crate::Readable for SDIO_ACT_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_act_conf::W`](W) writer structure"]
impl crate::Writable for SDIO_ACT_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_ACT_CONF to value 0"]
impl crate::Resettable for SDIO_ACT_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
