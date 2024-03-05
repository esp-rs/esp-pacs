#[doc = "Register `OUT_ETM_CONF_CH2` reader"]
pub type R = crate::R<OUT_ETM_CONF_CH2_SPEC>;
#[doc = "Register `OUT_ETM_CONF_CH2` writer"]
pub type W = crate::W<OUT_ETM_CONF_CH2_SPEC>;
#[doc = "Field `OUT_ETM_EN_CH2` reader - Set this bit to 1 to enable ETM task function"]
pub type OUT_ETM_EN_CH2_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN_CH2` writer - Set this bit to 1 to enable ETM task function"]
pub type OUT_ETM_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_LOOP_EN_CH2` reader - when this bit is 1, dscr can be processed after receiving a task"]
pub type OUT_ETM_LOOP_EN_CH2_R = crate::BitReader;
#[doc = "Field `OUT_ETM_LOOP_EN_CH2` writer - when this bit is 1, dscr can be processed after receiving a task"]
pub type OUT_ETM_LOOP_EN_CH2_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_MAK_CH2` reader - ETM dscr_ready maximum cache numbers"]
pub type OUT_DSCR_TASK_MAK_CH2_R = crate::FieldReader;
#[doc = "Field `OUT_DSCR_TASK_MAK_CH2` writer - ETM dscr_ready maximum cache numbers"]
pub type OUT_DSCR_TASK_MAK_CH2_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn out_etm_en_ch2(&self) -> OUT_ETM_EN_CH2_R {
        OUT_ETM_EN_CH2_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn out_etm_loop_en_ch2(&self) -> OUT_ETM_LOOP_EN_CH2_R {
        OUT_ETM_LOOP_EN_CH2_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn out_dscr_task_mak_ch2(&self) -> OUT_DSCR_TASK_MAK_CH2_R {
        OUT_DSCR_TASK_MAK_CH2_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("OUT_ETM_CONF_CH2")
            .field(
                "out_etm_en_ch2",
                &format_args!("{}", self.out_etm_en_ch2().bit()),
            )
            .field(
                "out_etm_loop_en_ch2",
                &format_args!("{}", self.out_etm_loop_en_ch2().bit()),
            )
            .field(
                "out_dscr_task_mak_ch2",
                &format_args!("{}", self.out_dscr_task_mak_ch2().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<OUT_ETM_CONF_CH2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    #[must_use]
    pub fn out_etm_en_ch2(&mut self) -> OUT_ETM_EN_CH2_W<OUT_ETM_CONF_CH2_SPEC> {
        OUT_ETM_EN_CH2_W::new(self, 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    #[must_use]
    pub fn out_etm_loop_en_ch2(&mut self) -> OUT_ETM_LOOP_EN_CH2_W<OUT_ETM_CONF_CH2_SPEC> {
        OUT_ETM_LOOP_EN_CH2_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    #[must_use]
    pub fn out_dscr_task_mak_ch2(&mut self) -> OUT_DSCR_TASK_MAK_CH2_W<OUT_ETM_CONF_CH2_SPEC> {
        OUT_DSCR_TASK_MAK_CH2_W::new(self, 2)
    }
}
#[doc = "TX CH2 ETM config register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`out_etm_conf_ch2::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`out_etm_conf_ch2::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct OUT_ETM_CONF_CH2_SPEC;
impl crate::RegisterSpec for OUT_ETM_CONF_CH2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`out_etm_conf_ch2::R`](R) reader structure"]
impl crate::Readable for OUT_ETM_CONF_CH2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`out_etm_conf_ch2::W`](W) writer structure"]
impl crate::Writable for OUT_ETM_CONF_CH2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets OUT_ETM_CONF_CH2 to value 0x04"]
impl crate::Resettable for OUT_ETM_CONF_CH2_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
