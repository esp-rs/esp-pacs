#[doc = "Register `R_PRIOR_VALUE%s` reader"]
pub type R = crate::R<R_PRIOR_VALUE_SPEC>;
#[doc = "Register `R_PRIOR_VALUE%s` writer"]
pub type W = crate::W<R_PRIOR_VALUE_SPEC>;
#[doc = "Field `REG_R_PRIOR_VALUE_0` reader - "]
pub type REG_R_PRIOR_VALUE_0_R = crate::FieldReader;
#[doc = "Field `REG_R_PRIOR_VALUE_0` writer - "]
pub type REG_R_PRIOR_VALUE_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_r_prior_value_0(&self) -> REG_R_PRIOR_VALUE_0_R {
        REG_R_PRIOR_VALUE_0_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("R_PRIOR_VALUE")
            .field("reg_r_prior_value_0", &self.reg_r_prior_value_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3"]
    #[inline(always)]
    pub fn reg_r_prior_value_0(&mut self) -> REG_R_PRIOR_VALUE_0_W<'_, R_PRIOR_VALUE_SPEC> {
        REG_R_PRIOR_VALUE_0_W::new(self, 0)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`r_prior_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`r_prior_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct R_PRIOR_VALUE_SPEC;
impl crate::RegisterSpec for R_PRIOR_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`r_prior_value::R`](R) reader structure"]
impl crate::Readable for R_PRIOR_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`r_prior_value::W`](W) writer structure"]
impl crate::Writable for R_PRIOR_VALUE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets R_PRIOR_VALUE%s to value 0"]
impl crate::Resettable for R_PRIOR_VALUE_SPEC {}
