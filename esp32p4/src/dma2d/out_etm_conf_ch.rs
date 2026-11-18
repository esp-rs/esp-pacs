#[doc = "Register `OUT_ETM_CONF_CH%s` reader"]
pub type R = crate::R<OUT_ETM_CONF_CH_SPEC>;
#[doc = "Register `OUT_ETM_CONF_CH%s` writer"]
pub type W = crate::W<OUT_ETM_CONF_CH_SPEC>;
#[doc = "Field `OUT_ETM_EN_CH` reader - Configures the enable of the etm function, 1 is enable."]
pub type OUT_ETM_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN_CH` writer - Configures the enable of the etm function, 1 is enable."]
pub type OUT_ETM_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_LOOP_EN_CH` reader - Configures the enable of the descriptors loop etm function, 1 is enable."]
pub type OUT_ETM_LOOP_EN_CH_R = crate::BitReader;
#[doc = "Field `OUT_ETM_LOOP_EN_CH` writer - Configures the enable of the descriptors loop etm function, 1 is enable."]
pub type OUT_ETM_LOOP_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_MAK_CH` reader - Configures the maximum number of cacheable descriptors."]
pub type OUT_DSCR_TASK_MAK_CH_R = crate::FieldReader;
#[doc = "Field `OUT_DSCR_TASK_MAK_CH` writer - Configures the maximum number of cacheable descriptors."]
pub type OUT_DSCR_TASK_MAK_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Configures the enable of the etm function, 1 is enable."]
    #[inline(always)]
    pub fn out_etm_en_ch(&self) -> OUT_ETM_EN_CH_R {
        OUT_ETM_EN_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the enable of the descriptors loop etm function, 1 is enable."]
    #[inline(always)]
    pub fn out_etm_loop_en_ch(&self) -> OUT_ETM_LOOP_EN_CH_R {
        OUT_ETM_LOOP_EN_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the maximum number of cacheable descriptors."]
    #[inline(always)]
    pub fn out_dscr_task_mak_ch(&self) -> OUT_DSCR_TASK_MAK_CH_R {
        OUT_DSCR_TASK_MAK_CH_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ETM_CONF_CH")
            .field("out_etm_en_ch", &self.out_etm_en_ch())
            .field("out_etm_loop_en_ch", &self.out_etm_loop_en_ch())
            .field("out_dscr_task_mak_ch", &self.out_dscr_task_mak_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the enable of the etm function, 1 is enable."]
    #[inline(always)]
    pub fn out_etm_en_ch(&mut self) -> OUT_ETM_EN_CH_W<'_, OUT_ETM_CONF_CH_SPEC> {
        OUT_ETM_EN_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the enable of the descriptors loop etm function, 1 is enable."]
    #[inline(always)]
    pub fn out_etm_loop_en_ch(&mut self) -> OUT_ETM_LOOP_EN_CH_W<'_, OUT_ETM_CONF_CH_SPEC> {
        OUT_ETM_LOOP_EN_CH_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the maximum number of cacheable descriptors."]
    #[inline(always)]
    pub fn out_dscr_task_mak_ch(&mut self) -> OUT_DSCR_TASK_MAK_CH_W<'_, OUT_ETM_CONF_CH_SPEC> {
        OUT_DSCR_TASK_MAK_CH_W::new(self, 2)
    }
}
#[doc = "Configures the tx etm of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`out_etm_conf_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`out_etm_conf_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ETM_CONF_CH_SPEC;
impl crate::RegisterSpec for OUT_ETM_CONF_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_etm_conf_ch::R`](R) reader structure"]
impl crate::Readable for OUT_ETM_CONF_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_etm_conf_ch::W`](W) writer structure"]
impl crate::Writable for OUT_ETM_CONF_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets OUT_ETM_CONF_CH%s to value 0x04"]
impl crate::Resettable for OUT_ETM_CONF_CH_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
