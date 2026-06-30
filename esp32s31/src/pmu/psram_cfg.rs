#[doc = "Register `PSRAM_CFG` reader"]
pub type R = crate::R<PSRAM_CFG_SPEC>;
#[doc = "Register `PSRAM_CFG` writer"]
pub type W = crate::W<PSRAM_CFG_SPEC>;
#[doc = "Field `PSRAM_XPD` reader - need_des"]
pub type PSRAM_XPD_R = crate::BitReader;
#[doc = "Field `PSRAM_XPD` writer - need_des"]
pub type PSRAM_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn psram_xpd(&self) -> PSRAM_XPD_R {
        PSRAM_XPD_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PSRAM_CFG")
            .field("psram_xpd", &self.psram_xpd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn psram_xpd(&mut self) -> PSRAM_XPD_W<'_, PSRAM_CFG_SPEC> {
        PSRAM_XPD_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`psram_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`psram_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PSRAM_CFG_SPEC;
impl crate::RegisterSpec for PSRAM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`psram_cfg::R`](R) reader structure"]
impl crate::Readable for PSRAM_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`psram_cfg::W`](W) writer structure"]
impl crate::Writable for PSRAM_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets PSRAM_CFG to value 0"]
impl crate::Resettable for PSRAM_CFG_SPEC {}
