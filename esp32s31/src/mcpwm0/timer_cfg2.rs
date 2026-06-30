#[doc = "Register `TIMER%s_CFG2` reader"]
pub type R = crate::R<TIMER_CFG2_SPEC>;
#[doc = "Register `TIMER%s_CFG2` writer"]
pub type W = crate::W<TIMER_CFG2_SPEC>;
#[doc = "Field `TIMER_A` reader - Configures the timestamp a shadow of PWM timer%s"]
pub type TIMER_A_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_A` writer - Configures the timestamp a shadow of PWM timer%s"]
pub type TIMER_A_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
#[doc = "Field `TIMER_A_UPMETHOD` reader - Configures the update method for active register of PWM timer%s timestamp a.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
pub type TIMER_A_UPMETHOD_R = crate::FieldReader;
#[doc = "Field `TIMER_A_UPMETHOD` writer - Configures the update method for active register of PWM timer%s timestamp a.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
pub type TIMER_A_UPMETHOD_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TIMER_A_DIR` reader - Configures the direction of the timer when timer%s value equal a value for event trigger.\\\\0: up \\\\1: down"]
pub type TIMER_A_DIR_R = crate::BitReader;
#[doc = "Field `TIMER_A_DIR` writer - Configures the direction of the timer when timer%s value equal a value for event trigger.\\\\0: up \\\\1: down"]
pub type TIMER_A_DIR_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:15 - Configures the timestamp a shadow of PWM timer%s"]
    #[inline(always)]
    pub fn timer_a(&self) -> TIMER_A_R {
        TIMER_A_R::new((self.bits & 0xffff) as u16)
    }
    #[doc = "Bits 16:17 - Configures the update method for active register of PWM timer%s timestamp a.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer_a_upmethod(&self) -> TIMER_A_UPMETHOD_R {
        TIMER_A_UPMETHOD_R::new(((self.bits >> 16) & 3) as u8)
    }
    #[doc = "Bit 18 - Configures the direction of the timer when timer%s value equal a value for event trigger.\\\\0: up \\\\1: down"]
    #[inline(always)]
    pub fn timer_a_dir(&self) -> TIMER_A_DIR_R {
        TIMER_A_DIR_R::new(((self.bits >> 18) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CFG2")
            .field("timer_a", &self.timer_a())
            .field("timer_a_upmethod", &self.timer_a_upmethod())
            .field("timer_a_dir", &self.timer_a_dir())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:15 - Configures the timestamp a shadow of PWM timer%s"]
    #[inline(always)]
    pub fn timer_a(&mut self) -> TIMER_A_W<'_, TIMER_CFG2_SPEC> {
        TIMER_A_W::new(self, 0)
    }
    #[doc = "Bits 16:17 - Configures the update method for active register of PWM timer%s timestamp a.\\\\0: Immediate\\\\1: TEZ\\\\2: Sync\\\\3: TEZ or sync\\\\TEZ here and below means timer equal zero event"]
    #[inline(always)]
    pub fn timer_a_upmethod(&mut self) -> TIMER_A_UPMETHOD_W<'_, TIMER_CFG2_SPEC> {
        TIMER_A_UPMETHOD_W::new(self, 16)
    }
    #[doc = "Bit 18 - Configures the direction of the timer when timer%s value equal a value for event trigger.\\\\0: up \\\\1: down"]
    #[inline(always)]
    pub fn timer_a_dir(&mut self) -> TIMER_A_DIR_W<'_, TIMER_CFG2_SPEC> {
        TIMER_A_DIR_W::new(self, 18)
    }
}
#[doc = "PWM timer%s timestamp a configuration register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cfg2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cfg2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CFG2_SPEC;
impl crate::RegisterSpec for TIMER_CFG2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_cfg2::R`](R) reader structure"]
impl crate::Readable for TIMER_CFG2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_cfg2::W`](W) writer structure"]
impl crate::Writable for TIMER_CFG2_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER%s_CFG2 to value 0xff"]
impl crate::Resettable for TIMER_CFG2_SPEC {
    const RESET_VALUE: u32 = 0xff;
}
