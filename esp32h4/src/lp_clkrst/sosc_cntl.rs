#[doc = "Register `SOSC_CNTL` reader"]
pub type R = crate::R<SOSC_CNTL_SPEC>;
#[doc = "Register `SOSC_CNTL` writer"]
pub type W = crate::W<SOSC_CNTL_SPEC>;
#[doc = "Field `SLOW_DFREQ` reader - Configures the RC_SLOW_CLK frequency,the clock frequency will increase with this field"]
pub type SLOW_DFREQ_R = crate::FieldReader;
#[doc = "Field `SLOW_DFREQ` writer - Configures the RC_SLOW_CLK frequency,the clock frequency will increase with this field"]
pub type SLOW_DFREQ_W<'a, REG> = crate::FieldWriter<'a, REG, 6>;
impl R {
    #[doc = "Bits 26:31 - Configures the RC_SLOW_CLK frequency,the clock frequency will increase with this field"]
    #[inline(always)]
    pub fn slow_dfreq(&self) -> SLOW_DFREQ_R {
        SLOW_DFREQ_R::new(((self.bits >> 26) & 0x3f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SOSC_CNTL")
            .field("slow_dfreq", &self.slow_dfreq())
            .finish()
    }
}
impl W {
    #[doc = "Bits 26:31 - Configures the RC_SLOW_CLK frequency,the clock frequency will increase with this field"]
    #[inline(always)]
    pub fn slow_dfreq(&mut self) -> SLOW_DFREQ_W<'_, SOSC_CNTL_SPEC> {
        SLOW_DFREQ_W::new(self, 26)
    }
}
#[doc = "Configures the RC_SLOW_CLK frequency\n\nYou can [`read`](crate::Reg::read) this register and get [`sosc_cntl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sosc_cntl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SOSC_CNTL_SPEC;
impl crate::RegisterSpec for SOSC_CNTL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sosc_cntl::R`](R) reader structure"]
impl crate::Readable for SOSC_CNTL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sosc_cntl::W`](W) writer structure"]
impl crate::Writable for SOSC_CNTL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SOSC_CNTL to value 0x2800_0000"]
impl crate::Resettable for SOSC_CNTL_SPEC {
    const RESET_VALUE: u32 = 0x2800_0000;
}
