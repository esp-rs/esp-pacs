#[doc = "Register `SDIO_WAKEUP_CNTL` reader"]
pub type R = crate::R<SDIO_WAKEUP_CNTL_SPEC>;
#[doc = "Register `SDIO_WAKEUP_CNTL` writer"]
pub type W = crate::W<SDIO_WAKEUP_CNTL_SPEC>;
#[doc = "Field `SDIO_ACT_DNUM` reader - need_des"]
pub type SDIO_ACT_DNUM_R = crate::FieldReader<u16>;
#[doc = "Field `SDIO_ACT_DNUM` writer - need_des"]
pub type SDIO_ACT_DNUM_W<'a, REG> = crate::FieldWriter<'a, REG, 10, u16>;
impl R {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn sdio_act_dnum(&self) -> SDIO_ACT_DNUM_R {
        SDIO_ACT_DNUM_R::new((self.bits & 0x03ff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SDIO_WAKEUP_CNTL")
            .field("sdio_act_dnum", &self.sdio_act_dnum())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:9 - need_des"]
    #[inline(always)]
    pub fn sdio_act_dnum(&mut self) -> SDIO_ACT_DNUM_W<SDIO_WAKEUP_CNTL_SPEC> {
        SDIO_ACT_DNUM_W::new(self, 0)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`sdio_wakeup_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sdio_wakeup_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SDIO_WAKEUP_CNTL_SPEC;
impl crate::RegisterSpec for SDIO_WAKEUP_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sdio_wakeup_cntl::R`](R) reader structure"]
impl crate::Readable for SDIO_WAKEUP_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sdio_wakeup_cntl::W`](W) writer structure"]
impl crate::Writable for SDIO_WAKEUP_CNTL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets SDIO_WAKEUP_CNTL to value 0x03ff"]
impl crate::Resettable for SDIO_WAKEUP_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x03ff;
}
