#[doc = "Register `INTERRUPT_RAW` reader"]
pub type R = crate::R<INTERRUPT_RAW_SPEC>;
#[doc = "Register `INTERRUPT_RAW` writer"]
pub type W = crate::W<INTERRUPT_RAW_SPEC>;
#[doc = "Field `REG_DEC_FAILURE_INT_RAW` reader - "]
pub type REG_DEC_FAILURE_INT_RAW_R = crate::BitReader;
#[doc = "Field `REG_DEC_FAILURE_INT_RAW` writer - "]
pub type REG_DEC_FAILURE_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `REG_TIMEOUT_INT_RAW` reader - "]
pub type REG_TIMEOUT_INT_RAW_R = crate::BitReader;
#[doc = "Field `REG_TIMEOUT_INT_RAW` writer - "]
pub type REG_TIMEOUT_INT_RAW_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_int_raw(&self) -> REG_DEC_FAILURE_INT_RAW_R {
        REG_DEC_FAILURE_INT_RAW_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_int_raw(&self) -> REG_TIMEOUT_INT_RAW_R {
        REG_TIMEOUT_INT_RAW_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INTERRUPT_RAW")
            .field("reg_dec_failure_int_raw", &self.reg_dec_failure_int_raw())
            .field("reg_timeout_int_raw", &self.reg_timeout_int_raw())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn reg_dec_failure_int_raw(&mut self) -> REG_DEC_FAILURE_INT_RAW_W<'_, INTERRUPT_RAW_SPEC> {
        REG_DEC_FAILURE_INT_RAW_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn reg_timeout_int_raw(&mut self) -> REG_TIMEOUT_INT_RAW_W<'_, INTERRUPT_RAW_SPEC> {
        REG_TIMEOUT_INT_RAW_W::new(self, 1)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`interrupt_raw::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`interrupt_raw::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INTERRUPT_RAW_SPEC;
impl crate::RegisterSpec for INTERRUPT_RAW_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`interrupt_raw::R`](R) reader structure"]
impl crate::Readable for INTERRUPT_RAW_SPEC {}
#[doc = "`write(|w| ..)` method takes [`interrupt_raw::W`](W) writer structure"]
impl crate::Writable for INTERRUPT_RAW_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets INTERRUPT_RAW to value 0"]
impl crate::Resettable for INTERRUPT_RAW_SPEC {}
