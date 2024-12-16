#[doc = "Register `U3_CHANGE_CONF` reader"]
pub type R = crate::R<U3_CHANGE_CONF_SPEC>;
#[doc = "Register `U3_CHANGE_CONF` writer"]
pub type W = crate::W<U3_CHANGE_CONF_SPEC>;
#[doc = "Field `CNT_STEP_U3` reader - Configures the step value for unit 3."]
pub type CNT_STEP_U3_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_U3` writer - Configures the step value for unit 3."]
pub type CNT_STEP_U3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_STEP_LIM_U3` reader - Configures the step limit value for unit 3."]
pub type CNT_STEP_LIM_U3_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_STEP_LIM_U3` writer - Configures the step limit value for unit 3."]
pub type CNT_STEP_LIM_U3_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the step value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_u3(&self) -> CNT_STEP_U3_R {
        CNT_STEP_U3_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_lim_u3(&self) -> CNT_STEP_LIM_U3_R {
        CNT_STEP_LIM_U3_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U3_CHANGE_CONF")
            .field("cnt_step_u3", &self.cnt_step_u3())
            .field("cnt_step_lim_u3", &self.cnt_step_lim_u3())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the step value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_u3(&mut self) -> CNT_STEP_U3_W<U3_CHANGE_CONF_SPEC> {
        CNT_STEP_U3_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the step limit value for unit 3."]
    #[inline(always)]
    pub fn cnt_step_lim_u3(&mut self) -> CNT_STEP_LIM_U3_W<U3_CHANGE_CONF_SPEC> {
        CNT_STEP_LIM_U3_W::new(self, 16)
    }
}
#[doc = "Configuration register for unit $n's step value.\n\nYou can [`read`](crate::Reg::read) this register and get [`u3_change_conf::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u3_change_conf::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U3_CHANGE_CONF_SPEC;
impl crate::RegisterSpec for U3_CHANGE_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u3_change_conf::R`](R) reader structure"]
impl crate::Readable for U3_CHANGE_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u3_change_conf::W`](W) writer structure"]
impl crate::Writable for U3_CHANGE_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets U3_CHANGE_CONF to value 0"]
impl crate::Resettable for U3_CHANGE_CONF_SPEC {
    const RESET_VALUE: u32 = 0;
}
