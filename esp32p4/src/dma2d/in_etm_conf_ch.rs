#[doc = "Register `IN_ETM_CONF_CH%s` reader"]
pub type R = crate::R<IN_ETM_CONF_CH_SPEC>;
#[doc = "Register `IN_ETM_CONF_CH%s` writer"]
pub type W = crate::W<IN_ETM_CONF_CH_SPEC>;
#[doc = "Field `IN_ETM_EN_CH` reader - Configures the enable of the etm function, 1 is enable."]
pub type IN_ETM_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_ETM_EN_CH` writer - Configures the enable of the etm function, 1 is enable."]
pub type IN_ETM_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ETM_LOOP_EN_CH` reader - Configures the enable of the descriptors loop etm function, 1 is enable."]
pub type IN_ETM_LOOP_EN_CH_R = crate::BitReader;
#[doc = "Field `IN_ETM_LOOP_EN_CH` writer - Configures the enable of the descriptors loop etm function, 1 is enable."]
pub type IN_ETM_LOOP_EN_CH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_TASK_MAK_CH` reader - Configures the maximum number of cacheable descriptors."]
pub type IN_DSCR_TASK_MAK_CH_R = crate::FieldReader;
#[doc = "Field `IN_DSCR_TASK_MAK_CH` writer - Configures the maximum number of cacheable descriptors."]
pub type IN_DSCR_TASK_MAK_CH_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Configures the enable of the etm function, 1 is enable."]
    #[inline(always)]
    pub fn in_etm_en_ch(&self) -> IN_ETM_EN_CH_R {
        IN_ETM_EN_CH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Configures the enable of the descriptors loop etm function, 1 is enable."]
    #[inline(always)]
    pub fn in_etm_loop_en_ch(&self) -> IN_ETM_LOOP_EN_CH_R {
        IN_ETM_LOOP_EN_CH_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - Configures the maximum number of cacheable descriptors."]
    #[inline(always)]
    pub fn in_dscr_task_mak_ch(&self) -> IN_DSCR_TASK_MAK_CH_R {
        IN_DSCR_TASK_MAK_CH_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("IN_ETM_CONF_CH")
            .field("in_etm_en_ch", &self.in_etm_en_ch())
            .field("in_etm_loop_en_ch", &self.in_etm_loop_en_ch())
            .field("in_dscr_task_mak_ch", &self.in_dscr_task_mak_ch())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Configures the enable of the etm function, 1 is enable."]
    #[inline(always)]
    pub fn in_etm_en_ch(&mut self) -> IN_ETM_EN_CH_W<'_, IN_ETM_CONF_CH_SPEC> {
        IN_ETM_EN_CH_W::new(self, 0)
    }
    #[doc = "Bit 1 - Configures the enable of the descriptors loop etm function, 1 is enable."]
    #[inline(always)]
    pub fn in_etm_loop_en_ch(&mut self) -> IN_ETM_LOOP_EN_CH_W<'_, IN_ETM_CONF_CH_SPEC> {
        IN_ETM_LOOP_EN_CH_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - Configures the maximum number of cacheable descriptors."]
    #[inline(always)]
    pub fn in_dscr_task_mak_ch(&mut self) -> IN_DSCR_TASK_MAK_CH_W<'_, IN_ETM_CONF_CH_SPEC> {
        IN_DSCR_TASK_MAK_CH_W::new(self, 2)
    }
}
#[doc = "Configures the rx etm of channel %s\n\nYou can [`read`](crate::Reg::read) this register and get [`in_etm_conf_ch::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`in_etm_conf_ch::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct IN_ETM_CONF_CH_SPEC;
impl crate::RegisterSpec for IN_ETM_CONF_CH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`in_etm_conf_ch::R`](R) reader structure"]
impl crate::Readable for IN_ETM_CONF_CH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`in_etm_conf_ch::W`](W) writer structure"]
impl crate::Writable for IN_ETM_CONF_CH_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets IN_ETM_CONF_CH%s to value 0x04"]
impl crate::Resettable for IN_ETM_CONF_CH_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
