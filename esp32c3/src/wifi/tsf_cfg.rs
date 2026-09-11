#[doc = "Register `TSF_CFG%s` reader"]
pub type R = crate::R<TSF_CFG_SPEC>;
#[doc = "Register `TSF_CFG%s` writer"]
pub type W = crate::W<TSF_CFG_SPEC>;
#[doc = "Field `TBTT_ENABLE` reader - Set by tsf_hal_set_tbtt_enable"]
pub type TBTT_ENABLE_R = crate::BitReader;
#[doc = "Field `TBTT_ENABLE` writer - Set by tsf_hal_set_tbtt_enable"]
pub type TBTT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TSF_ENABLE_AUX` reader - Bits 27 and 28 are set together with TSF_ENABLE by tsf_hal_set_tsf_enable, meaning unknown"]
pub type TSF_ENABLE_AUX_R = crate::FieldReader;
#[doc = "Field `TSF_ENABLE_AUX` writer - Bits 27 and 28 are set together with TSF_ENABLE by tsf_hal_set_tsf_enable, meaning unknown"]
pub type TSF_ENABLE_AUX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TSF_ENABLE` reader - Enables the TSF counter, checked by tsf_hal_is_tsf_enabled"]
pub type TSF_ENABLE_R = crate::BitReader;
#[doc = "Field `TSF_ENABLE` writer - Enables the TSF counter, checked by tsf_hal_is_tsf_enabled"]
pub type TSF_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 26 - Set by tsf_hal_set_tbtt_enable"]
    #[inline(always)]
    pub fn tbtt_enable(&self) -> TBTT_ENABLE_R {
        TBTT_ENABLE_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bits 27:28 - Bits 27 and 28 are set together with TSF_ENABLE by tsf_hal_set_tsf_enable, meaning unknown"]
    #[inline(always)]
    pub fn tsf_enable_aux(&self) -> TSF_ENABLE_AUX_R {
        TSF_ENABLE_AUX_R::new(((self.bits >> 27) & 3) as u8)
    }
    #[doc = "Bit 31 - Enables the TSF counter, checked by tsf_hal_is_tsf_enabled"]
    #[inline(always)]
    pub fn tsf_enable(&self) -> TSF_ENABLE_R {
        TSF_ENABLE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TSF_CFG")
            .field("tbtt_enable", &self.tbtt_enable())
            .field("tsf_enable_aux", &self.tsf_enable_aux())
            .field("tsf_enable", &self.tsf_enable())
            .finish()
    }
}
impl W {
    #[doc = "Bit 26 - Set by tsf_hal_set_tbtt_enable"]
    #[inline(always)]
    pub fn tbtt_enable(&mut self) -> TBTT_ENABLE_W<'_, TSF_CFG_SPEC> {
        TBTT_ENABLE_W::new(self, 26)
    }
    #[doc = "Bits 27:28 - Bits 27 and 28 are set together with TSF_ENABLE by tsf_hal_set_tsf_enable, meaning unknown"]
    #[inline(always)]
    pub fn tsf_enable_aux(&mut self) -> TSF_ENABLE_AUX_W<'_, TSF_CFG_SPEC> {
        TSF_ENABLE_AUX_W::new(self, 27)
    }
    #[doc = "Bit 31 - Enables the TSF counter, checked by tsf_hal_is_tsf_enabled"]
    #[inline(always)]
    pub fn tsf_enable(&mut self) -> TSF_ENABLE_W<'_, TSF_CFG_SPEC> {
        TSF_ENABLE_W::new(self, 31)
    }
}
#[doc = "TSF and TBTT configuration of an interface\n\nYou can [`read`](crate::Reg::read) this register and get [`tsf_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`tsf_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TSF_CFG_SPEC;
impl crate::RegisterSpec for TSF_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`tsf_cfg::R`](R) reader structure"]
impl crate::Readable for TSF_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`tsf_cfg::W`](W) writer structure"]
impl crate::Writable for TSF_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TSF_CFG%s to value 0"]
impl crate::Resettable for TSF_CFG_SPEC {}
