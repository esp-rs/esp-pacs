#[doc = "Register `INTERRUPT_ENA` reader"]
pub type R = crate::R<INTERRUPT_ENA_SPEC>;
#[doc = "Register `INTERRUPT_ENA` writer"]
pub type W = crate::W<INTERRUPT_ENA_SPEC>;
#[doc = "Field `REG_DEC_FAILURE_INT_ENA` reader - "]
pub type REG_DEC_FAILURE_INT_ENA_R = crate::BitReader;
#[doc = "Field `REG_DEC_FAILURE_INT_ENA` writer - "]
pub type REG_DEC_FAILURE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMEOUT_INT_ENA` reader - "]
pub type REG_TIMEOUT_INT_ENA_R = crate::BitReader;
#[doc = "Field `REG_TIMEOUT_INT_ENA` writer - "]
pub type REG_TIMEOUT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_int_ena(&self) -> REG_DEC_FAILURE_INT_ENA_R {
        REG_DEC_FAILURE_INT_ENA_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_int_ena(&self) -> REG_TIMEOUT_INT_ENA_R {
        REG_TIMEOUT_INT_ENA_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_ENA")
            .field("reg_dec_failure_int_ena", &self.reg_dec_failure_int_ena())
            .field("reg_timeout_int_ena", &self.reg_timeout_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_int_ena(&mut self) -> REG_DEC_FAILURE_INT_ENA_W<'_, INTERRUPT_ENA_SPEC> {
        REG_DEC_FAILURE_INT_ENA_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_int_ena(&mut self) -> REG_TIMEOUT_INT_ENA_W<'_, INTERRUPT_ENA_SPEC> {
        REG_TIMEOUT_INT_ENA_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_ENA_SPEC;
impl crate::RegisterSpec for INTERRUPT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_ena::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_ena::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_ENA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_ENA to value 0"]
impl crate::Resettable for INTERRUPT_ENA_SPEC {}
