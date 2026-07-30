#[doc = "Register `U0_CONF3` reader"]
pub type R = crate::R<U0_CONF3_SPEC>;
#[doc = "Register `U0_CONF3` writer"]
pub type W = crate::W<U0_CONF3_SPEC>;
#[doc = "Field `CNT_H_STEP_U0` reader - Configures the forward rotation step value for unit 0."]
pub type CNT_H_STEP_U0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_H_STEP_U0` writer - Configures the forward rotation step value for unit 0."]
pub type CNT_H_STEP_U0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `CNT_L_STEP_U0` reader - Configures the reverse rotation step value for unit 0."]
pub type CNT_L_STEP_U0_R = crate::FieldReader<u16>;
#[doc = "Field `CNT_L_STEP_U0` writer - Configures the reverse rotation step value for unit 0."]
pub type CNT_L_STEP_U0_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bits 0:15 - Configures the forward rotation step value for unit 0."]
    #[inline(always)]
    pub fn cnt_h_step_u0(&self) -> CNT_H_STEP_U0_R {
        CNT_H_STEP_U0_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:31 - Configures the reverse rotation step value for unit 0."]
    #[inline(always)]
    pub fn cnt_l_step_u0(&self) -> CNT_L_STEP_U0_R {
        CNT_L_STEP_U0_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("U0_CONF3")
            .field("cnt_h_step_u0", &self.cnt_h_step_u0())
            .field("cnt_l_step_u0", &self.cnt_l_step_u0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the forward rotation step value for unit 0."]
    #[inline(always)]
    pub fn cnt_h_step_u0(&mut self) -> CNT_H_STEP_U0_W<'_, U0_CONF3_SPEC> {
        CNT_H_STEP_U0_W::new(self, 0)
    }
    #[doc = "Bits 16:31 - Configures the reverse rotation step value for unit 0."]
    #[inline(always)]
    pub fn cnt_l_step_u0(&mut self) -> CNT_L_STEP_U0_W<'_, U0_CONF3_SPEC> {
        CNT_L_STEP_U0_W::new(self, 16)
    }
}
#[doc = "Configuration register for unit $n's step value.\n\nYou can [`read`](crate::Reg::read) this register and get [`u0_conf3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`u0_conf3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct U0_CONF3_SPEC;
impl crate::RegisterSpec for U0_CONF3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`u0_conf3::R`](R) reader structure"]
impl crate::Readable for U0_CONF3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`u0_conf3::W`](W) writer structure"]
impl crate::Writable for U0_CONF3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets U0_CONF3 to value 0"]
impl crate::Resettable for U0_CONF3_SPEC {}
