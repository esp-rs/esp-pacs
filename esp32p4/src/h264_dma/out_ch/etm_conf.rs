#[doc = "Register `ETM_CONF` reader"]
pub type R = crate::R<ETM_CONF_SPEC>;
#[doc = "Register `ETM_CONF` writer"]
pub type W = crate::W<ETM_CONF_SPEC>;
#[doc = "Field `OUT_ETM_EN` reader - Set this bit to 1 to enable ETM task function"]
pub type OUT_ETM_EN_R = crate::BitReader;
#[doc = "Field `OUT_ETM_EN` writer - Set this bit to 1 to enable ETM task function"]
pub type OUT_ETM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_ETM_LOOP_EN` reader - when this bit is 1, dscr can be processed after receiving a task"]
pub type OUT_ETM_LOOP_EN_R = crate::BitReader;
#[doc = "Field `OUT_ETM_LOOP_EN` writer - when this bit is 1, dscr can be processed after receiving a task"]
pub type OUT_ETM_LOOP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OUT_DSCR_TASK_MAK` reader - ETM dscr_ready maximum cache numbers"]
pub type OUT_DSCR_TASK_MAK_R = crate::FieldReader;
#[doc = "Field `OUT_DSCR_TASK_MAK` writer - ETM dscr_ready maximum cache numbers"]
pub type OUT_DSCR_TASK_MAK_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn out_etm_en(&self) -> OUT_ETM_EN_R {
        OUT_ETM_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn out_etm_loop_en(&self) -> OUT_ETM_LOOP_EN_R {
        OUT_ETM_LOOP_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn out_dscr_task_mak(&self) -> OUT_DSCR_TASK_MAK_R {
        OUT_DSCR_TASK_MAK_R::new(((self.bits >> 2) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ETM_CONF")
            .field("out_etm_en", &self.out_etm_en())
            .field("out_etm_loop_en", &self.out_etm_loop_en())
            .field("out_dscr_task_mak", &self.out_dscr_task_mak())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to 1 to enable ETM task function"]
    #[inline(always)]
    pub fn out_etm_en(&mut self) -> OUT_ETM_EN_W<'_, ETM_CONF_SPEC> {
        OUT_ETM_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - when this bit is 1, dscr can be processed after receiving a task"]
    #[inline(always)]
    pub fn out_etm_loop_en(&mut self) -> OUT_ETM_LOOP_EN_W<'_, ETM_CONF_SPEC> {
        OUT_ETM_LOOP_EN_W::new(self, 1)
    }
    #[doc = "Bits 2:3 - ETM dscr_ready maximum cache numbers"]
    #[inline(always)]
    pub fn out_dscr_task_mak(&mut self) -> OUT_DSCR_TASK_MAK_W<'_, ETM_CONF_SPEC> {
        OUT_DSCR_TASK_MAK_W::new(self, 2)
    }
}
#[doc = "TX CHx ETM config register\n\nYou can [`read`](crate::Reg::read) this register and get [`etm_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`etm_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ETM_CONF_SPEC;
impl crate::RegisterSpec for ETM_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`etm_conf::R`](R) reader structure"]
impl crate::Readable for ETM_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`etm_conf::W`](W) writer structure"]
impl crate::Writable for ETM_CONF_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ETM_CONF to value 0x04"]
impl crate::Resettable for ETM_CONF_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
