#[doc = "Register `INT_THRESH` reader"]
pub type R = crate::R<INT_THRESH_SPEC>;
#[doc = "Register `INT_THRESH` writer"]
pub type W = crate::W<INT_THRESH_SPEC>;
#[doc = "Field `CPU_INT_THRESH` reader - CPU interrupt threshold level"]
pub type CPU_INT_THRESH_R = crate::FieldReader;
#[doc = "Field `CPU_INT_THRESH` writer - CPU interrupt threshold level"]
pub type CPU_INT_THRESH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 24:31 - CPU interrupt threshold level"]
    #[inline(always)]
    pub fn cpu_int_thresh(&self) -> CPU_INT_THRESH_R {
        CPU_INT_THRESH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_THRESH")
            .field("cpu_int_thresh", &self.cpu_int_thresh())
            .finish()
    }
}
impl W {
    #[doc = "Bits 24:31 - CPU interrupt threshold level"]
    #[inline(always)]
    #[must_use]
    pub fn cpu_int_thresh(&mut self) -> CPU_INT_THRESH_W<INT_THRESH_SPEC> {
        CPU_INT_THRESH_W::new(self, 24)
    }
}
#[doc = "Interrupt threshold register\n\nYou can [`read`](crate::Reg::read) this register and get [`int_thresh::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`int_thresh::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_THRESH_SPEC;
impl crate::RegisterSpec for INT_THRESH_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_thresh::R`](R) reader structure"]
impl crate::Readable for INT_THRESH_SPEC {}
#[doc = "`write(|w| ..)` method takes [`int_thresh::W`](W) writer structure"]
impl crate::Writable for INT_THRESH_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets INT_THRESH to value 0"]
impl crate::Resettable for INT_THRESH_SPEC {
    const RESET_VALUE: u32 = 0;
}
