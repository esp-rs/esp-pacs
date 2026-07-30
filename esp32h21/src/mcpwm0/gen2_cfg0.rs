#[doc = "Register `GEN2_CFG0` reader"]
pub type R = crate::R<GEN2_CFG0_SPEC>;
#[doc = "Register `GEN2_CFG0` writer"]
pub type W = crate::W<GEN2_CFG0_SPEC>;
#[doc = "Field `GEN2_CFG_UPMETHOD` reader - Update method for PWM generator 2's active register of configuration. 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:sync;when bit3 is set to 1:disable the update."]
pub type GEN2_CFG_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN2_CFG_UPMETHOD` writer - Update method for PWM generator 2's active register of configuration. 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:sync;when bit3 is set to 1:disable the update."]
pub type GEN2_CFG_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GEN2_T0_SEL` reader - Source selection for PWM generator 2 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN2_T0_SEL_R = crate::FieldReader;
#[doc = "Field `GEN2_T0_SEL` writer - Source selection for PWM generator 2 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN2_T0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GEN2_T1_SEL` reader - Source selection for PWM generator 2 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN2_T1_SEL_R = crate::FieldReader;
#[doc = "Field `GEN2_T1_SEL` writer - Source selection for PWM generator 2 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
pub type GEN2_T1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Update method for PWM generator 2's active register of configuration. 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:sync;when bit3 is set to 1:disable the update."]
    #[inline(always)]
    pub fn gen2_cfg_upmethod(&self) -> GEN2_CFG_UPMETHOD_R {
        GEN2_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Source selection for PWM generator 2 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    pub fn gen2_t0_sel(&self) -> GEN2_T0_SEL_R {
        GEN2_T0_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Source selection for PWM generator 2 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    pub fn gen2_t1_sel(&self) -> GEN2_T1_SEL_R {
        GEN2_T1_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN2_CFG0")
            .field("gen2_cfg_upmethod", &self.gen2_cfg_upmethod())
            .field("gen2_t0_sel", &self.gen2_t0_sel())
            .field("gen2_t1_sel", &self.gen2_t1_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Update method for PWM generator 2's active register of configuration. 0: immediately, when bit0 is set to 1: TEZ, when bit1 is set to 1:sync;when bit3 is set to 1:disable the update."]
    #[inline(always)]
    pub fn gen2_cfg_upmethod(&mut self) -> GEN2_CFG_UPMETHOD_W<'_, GEN2_CFG0_SPEC> {
        GEN2_CFG_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Source selection for PWM generator 2 event_t0, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    pub fn gen2_t0_sel(&mut self) -> GEN2_T0_SEL_W<'_, GEN2_CFG0_SPEC> {
        GEN2_T0_SEL_W::new(self, 4)
    }
    #[doc = "Bits 7:9 - Source selection for PWM generator 2 event_t1, take effect immediately, 0: fault_event0, 1: fault_event1, 2: fault_event2, 3: sync_taken, 4: none"]
    #[inline(always)]
    pub fn gen2_t1_sel(&mut self) -> GEN2_T1_SEL_W<'_, GEN2_CFG0_SPEC> {
        GEN2_T1_SEL_W::new(self, 7)
    }
}
#[doc = "Fault event T0 and T1 handling\n\nYou can [`read`](crate::Reg::read) this register and get [`gen2_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen2_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN2_CFG0_SPEC;
impl crate::RegisterSpec for GEN2_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen2_cfg0::R`](R) reader structure"]
impl crate::Readable for GEN2_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen2_cfg0::W`](W) writer structure"]
impl crate::Writable for GEN2_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN2_CFG0 to value 0"]
impl crate::Resettable for GEN2_CFG0_SPEC {}
