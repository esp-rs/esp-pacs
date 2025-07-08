#[doc = "Register `GEN%s_CFG0` reader"]
pub type R = crate::R<GEN_CFG0_SPEC>;
#[doc = "Register `GEN%s_CFG0` writer"]
pub type W = crate::W<GEN_CFG0_SPEC>;
#[doc = "Field `GEN_CFG_UPMETHOD` reader - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type GEN_CFG_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `GEN_CFG_UPMETHOD` writer - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
pub type GEN_CFG_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `GEN_T0_SEL` reader - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type GEN_T0_SEL_R = crate::FieldReader;
#[doc = "Field `GEN_T0_SEL` writer - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type GEN_T0_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `GEN_T1_SEL` reader - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type GEN_T1_SEL_R = crate::FieldReader;
#[doc = "Field `GEN_T1_SEL` writer - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
pub type GEN_T1_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = "Bits 0:3 - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn gen_cfg_upmethod(&self) -> GEN_CFG_UPMETHOD_R {
        GEN_CFG_UPMETHOD_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:6 - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn gen_t0_sel(&self) -> GEN_T0_SEL_R {
        GEN_T0_SEL_R::new(((self.bits >> 4) & 7) as u8)
    }
    #[doc = "Bits 7:9 - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn gen_t1_sel(&self) -> GEN_T1_SEL_R {
        GEN_T1_SEL_R::new(((self.bits >> 7) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_CFG0")
            .field("gen_cfg_upmethod", &self.gen_cfg_upmethod())
            .field("gen_t0_sel", &self.gen_t0_sel())
            .field("gen_t1_sel", &self.gen_t1_sel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - Configures update method for PWM generator %s's active register.\\\\0: Immediately\\\\Bit0 is set to 1: TEZ\\\\Bit1 is set to 1: TEP\\\\Bit2 is set to 1: Sync\\\\Bit3 is set to 1: Disable the update"]
    #[inline(always)]
    pub fn gen_cfg_upmethod(&mut self) -> GEN_CFG_UPMETHOD_W<GEN_CFG0_SPEC> {
        GEN_CFG_UPMETHOD_W::new(self, 0)
    }
    #[doc = "Bits 4:6 - Configures source selection for PWM generator %s event_t0, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn gen_t0_sel(&mut self) -> GEN_T0_SEL_W<GEN_CFG0_SPEC> {
        GEN_T0_SEL_W::new(self, 4)
    }
    #[doc = "Bits 7:9 - Configures source selection for PWM generator %s event_t1, take effect immediately.\\\\0: fault_event0\\\\1: fault_event1\\\\2: fault_event2\\\\3: sync_taken\\\\4: Invalid, Select nothing"]
    #[inline(always)]
    pub fn gen_t1_sel(&mut self) -> GEN_T1_SEL_W<GEN_CFG0_SPEC> {
        GEN_T1_SEL_W::new(self, 7)
    }
}
#[doc = "Generator%s fault event T0 and T1 configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_cfg0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_cfg0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_CFG0_SPEC;
impl crate::RegisterSpec for GEN_CFG0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_cfg0::R`](R) reader structure"]
impl crate::Readable for GEN_CFG0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_cfg0::W`](W) writer structure"]
impl crate::Writable for GEN_CFG0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets GEN%s_CFG0 to value 0"]
impl crate::Resettable for GEN_CFG0_SPEC {}
