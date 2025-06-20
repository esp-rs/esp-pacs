#[doc = "Register `TIMER_LD_VAL_L` reader"]
pub type R = crate::R<TIMER_LD_VAL_L_SPEC>;
#[doc = "Register `TIMER_LD_VAL_L` writer"]
pub type W = crate::W<TIMER_LD_VAL_L_SPEC>;
#[doc = "Field `TIMER_LD_VAL_L` reader - TWAI FD timer count pre-load value register, low part."]
pub type TIMER_LD_VAL_L_R = crate::FieldReader<u32>;
#[doc = "Field `TIMER_LD_VAL_L` writer - TWAI FD timer count pre-load value register, low part."]
pub type TIMER_LD_VAL_L_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    #[doc = "Bits 0:31 - TWAI FD timer count pre-load value register, low part."]
    #[inline(always)]
    pub fn timer_ld_val_l(&self) -> TIMER_LD_VAL_L_R {
        TIMER_LD_VAL_L_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_LD_VAL_L")
            .field("timer_ld_val_l", &self.timer_ld_val_l())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:31 - TWAI FD timer count pre-load value register, low part."]
    #[inline(always)]
    pub fn timer_ld_val_l(&mut self) -> TIMER_LD_VAL_L_W<TIMER_LD_VAL_L_SPEC> {
        TIMER_LD_VAL_L_W::new(self, 0)
    }
}
#[doc = "TWAI FD timer pre-load value low register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_ld_val_l::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_ld_val_l::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_LD_VAL_L_SPEC;
impl crate::RegisterSpec for TIMER_LD_VAL_L_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_ld_val_l::R`](R) reader structure"]
impl crate::Readable for TIMER_LD_VAL_L_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_ld_val_l::W`](W) writer structure"]
impl crate::Writable for TIMER_LD_VAL_L_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets TIMER_LD_VAL_L to value 0"]
impl crate::Resettable for TIMER_LD_VAL_L_SPEC {
    const RESET_VALUE: u32 = 0;
}
