#[doc = "Register `CONF3` reader"]
pub type R = crate::R<CONF3_SPEC>;
#[doc = "Register `CONF3` writer"]
pub type W = crate::W<CONF3_SPEC>;
#[doc = "Field `CNT_STEP` reader - Configures the step value for unit %s."]
pub type CNT_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP` writer - Configures the step value for unit %s."]
pub type CNT_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_STEP_LIM` reader - Configures the step limit value for unit %s."]
pub type CNT_STEP_LIM_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_LIM` writer - Configures the step limit value for unit %s."]
pub type CNT_STEP_LIM_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the step value for unit %s."]
    #[inline(always)]
    pub fn cnt_step(&self) -> CNT_STEP_R {
        CNT_STEP_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit %s."]
    #[inline(always)]
    pub fn cnt_step_lim(&self) -> CNT_STEP_LIM_R {
        CNT_STEP_LIM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CONF3")
            .field("cnt_step", &self.cnt_step())
            .field("cnt_step_lim", &self.cnt_step_lim())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the step value for unit %s."]
    #[inline(always)]
    pub fn cnt_step(&mut self) -> CNT_STEP_W<'_, CONF3_SPEC> {
        CNT_STEP_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit %s."]
    #[inline(always)]
    pub fn cnt_step_lim(&mut self) -> CNT_STEP_LIM_W<'_, CONF3_SPEC> {
        CNT_STEP_LIM_W::new(self, 16)
    }
}
#[doc = "Configuration register for unit 0's step value.\n\nYou can [`read`](crate::Reg::read) this register and get [`conf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`conf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CONF3_SPEC;
impl crate::RegisterSpec for CONF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`conf3::R`](R) reader structure"]
impl crate::Readable for CONF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`conf3::W`](W) writer structure"]
impl crate::Writable for CONF3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CONF3 to value 0"]
impl crate::Resettable for CONF3_SPEC {}
