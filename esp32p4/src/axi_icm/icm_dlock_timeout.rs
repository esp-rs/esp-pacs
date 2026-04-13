#[doc = "Register `ICM_DLOCK_TIMEOUT` reader"]
pub type R = crate::R<ICM_DLOCK_TIMEOUT_SPEC>;
#[doc = "Register `ICM_DLOCK_TIMEOUT` writer"]
pub type W = crate::W<ICM_DLOCK_TIMEOUT_SPEC>;
#[doc = "Field `ICM_REG_DLOCK_TIMEOUT` reader - "]
pub type ICM_REG_DLOCK_TIMEOUT_R = crate::FieldReader<u16>;
#[doc = "Field `ICM_REG_DLOCK_TIMEOUT` writer - "]
pub type ICM_REG_DLOCK_TIMEOUT_W<'a, REG> = crate::FieldWriter<'a, REG, 13, u16>;
impl R {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn icm_reg_dlock_timeout(&self) -> ICM_REG_DLOCK_TIMEOUT_R {
        ICM_REG_DLOCK_TIMEOUT_R::new((self.bits & 0x1fff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ICM_DLOCK_TIMEOUT")
            .field("icm_reg_dlock_timeout", &self.icm_reg_dlock_timeout())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:12"]
    #[inline(always)]
    pub fn icm_reg_dlock_timeout(&mut self) -> ICM_REG_DLOCK_TIMEOUT_W<'_, ICM_DLOCK_TIMEOUT_SPEC> {
        ICM_REG_DLOCK_TIMEOUT_W::new(self, 0)
    }
}
#[doc = "Deadlock timeout\n\nYou can [`read`](crate::Reg::read) this register and get [`icm_dlock_timeout::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`icm_dlock_timeout::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ICM_DLOCK_TIMEOUT_SPEC;
impl crate::RegisterSpec for ICM_DLOCK_TIMEOUT_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`icm_dlock_timeout::R`](R) reader structure"]
impl crate::Readable for ICM_DLOCK_TIMEOUT_SPEC {}
#[doc = "`write(|w| ..)` method takes [`icm_dlock_timeout::W`](W) writer structure"]
impl crate::Writable for ICM_DLOCK_TIMEOUT_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ICM_DLOCK_TIMEOUT to value 0"]
impl crate::Resettable for ICM_DLOCK_TIMEOUT_SPEC {}
