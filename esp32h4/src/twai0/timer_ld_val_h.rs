#[doc = "Register `TIMER_LD_VAL_H` reader"]
pub type R = crate::R<TIMER_LD_VAL_H_SPEC>;
#[doc = "Register `TIMER_LD_VAL_H` writer"]
pub type W = crate::W<TIMER_LD_VAL_H_SPEC>;
#[doc = "Field `TIMER_LD_VAL_H` reader - TWAI FD timer pre-load value register, high part. If timestamp valid bit-width less than 33, this field is ignored."]
pub type TIMER_LD_VAL_H_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_LD_VAL_H` writer - TWAI FD timer pre-load value register, high part. If timestamp valid bit-width less than 33, this field is ignored."]
pub type TIMER_LD_VAL_H_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TWAI FD timer pre-load value register, high part. If timestamp valid bit-width less than 33, this field is ignored."]
    #[inline(always)]
    pub fn timer_ld_val_h(&self) -> TIMER_LD_VAL_H_R {
        TIMER_LD_VAL_H_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_LD_VAL_H")
            .field("timer_ld_val_h", &self.timer_ld_val_h())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - TWAI FD timer pre-load value register, high part. If timestamp valid bit-width less than 33, this field is ignored."]
    #[inline(always)]
    pub fn timer_ld_val_h(&mut self) -> TIMER_LD_VAL_H_W<'_, TIMER_LD_VAL_H_SPEC> {
        TIMER_LD_VAL_H_W::new(self, 0)
    }
}
#[doc = "TWAI FD timer pre-load value high register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ld_val_h::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ld_val_h::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_LD_VAL_H_SPEC;
impl crate::RegisterSpec for TIMER_LD_VAL_H_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ld_val_h::R`](R) reader structure"]
impl crate::Readable for TIMER_LD_VAL_H_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_ld_val_h::W`](W) writer structure"]
impl crate::Writable for TIMER_LD_VAL_H_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_LD_VAL_H to value 0"]
impl crate::Resettable for TIMER_LD_VAL_H_SPEC {}
