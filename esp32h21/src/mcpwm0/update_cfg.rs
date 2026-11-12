#[doc = "Register `UPDATE_CFG` reader"]
pub type R = crate::R<UPDATE_CFG_SPEC>;
#[doc = "Register `UPDATE_CFG` writer"]
pub type W = crate::W<UPDATE_CFG_SPEC>;
#[doc = "Field `GLOBAL_UP_EN` reader - The global enable of update of all active registers in MCPWM module"]
pub type GLOBAL_UP_EN_R = crate::BitReader;
#[doc = "Field `GLOBAL_UP_EN` writer - The global enable of update of all active registers in MCPWM module"]
pub type GLOBAL_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `GLOBAL_FORCE_UP` reader - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
pub type GLOBAL_FORCE_UP_R = crate::BitReader;
#[doc = "Field `GLOBAL_FORCE_UP` writer - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
pub type GLOBAL_FORCE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP_UP_EN(0-2)` reader - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator %s are enabled"]
pub type OP_UP_EN_R = crate::BitReader;
#[doc = "Field `OP_UP_EN(0-2)` writer - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator %s are enabled"]
pub type OP_UP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `OP_FORCE_UP(0-2)` reader - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator %s"]
pub type OP_FORCE_UP_R = crate::BitReader;
#[doc = "Field `OP_FORCE_UP(0-2)` writer - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator %s"]
pub type OP_FORCE_UP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&self) -> GLOBAL_UP_EN_R {
        GLOBAL_UP_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&self) -> GLOBAL_FORCE_UP_R {
        GLOBAL_FORCE_UP_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator (0-2) are enabled"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn op_up_en(&self, n: u8) -> OP_UP_EN_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_UP_EN_R::new(((self.bits >> (n * 2 + 2)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator (0-2) are enabled"]
    #[inline(always)]
    pub fn op_up_en_iter(&self) -> impl Iterator<Item = OP_UP_EN_R> + '_ {
        (0..3).map(move |n| OP_UP_EN_R::new(((self.bits >> (n * 2 + 2)) & 1) != 0))
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&self) -> OP_UP_EN_R {
        OP_UP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&self) -> OP_UP_EN_R {
        OP_UP_EN_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&self) -> OP_UP_EN_R {
        OP_UP_EN_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "A toggle (software invert its value) will trigger a forced update of active registers in PWM operator (0-2)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_FORCE_UP` field.</div>"]
    #[inline(always)]
    pub fn op_force_up(&self, n: u8) -> OP_FORCE_UP_R {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_FORCE_UP_R::new(((self.bits >> (n * 2 + 3)) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "A toggle (software invert its value) will trigger a forced update of active registers in PWM operator (0-2)"]
    #[inline(always)]
    pub fn op_force_up_iter(&self) -> impl Iterator<Item = OP_FORCE_UP_R> + '_ {
        (0..3).map(move |n| OP_FORCE_UP_R::new(((self.bits >> (n * 2 + 3)) & 1) != 0))
    }
    #[doc = "Bit 3 - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&self) -> OP_FORCE_UP_R {
        OP_FORCE_UP_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 5 - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&self) -> OP_FORCE_UP_R {
        OP_FORCE_UP_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 7 - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&self) -> OP_FORCE_UP_R {
        OP_FORCE_UP_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UPDATE_CFG")
            .field("global_up_en", &self.global_up_en())
            .field("global_force_up", &self.global_force_up())
            .field("op0_up_en", &self.op0_up_en())
            .field("op1_up_en", &self.op1_up_en())
            .field("op2_up_en", &self.op2_up_en())
            .field("op0_force_up", &self.op0_force_up())
            .field("op1_force_up", &self.op1_force_up())
            .field("op2_force_up", &self.op2_force_up())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - The global enable of update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_up_en(&mut self) -> GLOBAL_UP_EN_W<'_, UPDATE_CFG_SPEC> {
        GLOBAL_UP_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - a toggle (software invert its value) will trigger a forced update of all active registers in MCPWM module"]
    #[inline(always)]
    pub fn global_force_up(&mut self) -> GLOBAL_FORCE_UP_W<'_, UPDATE_CFG_SPEC> {
        GLOBAL_FORCE_UP_W::new(self, 1)
    }
    #[doc = "When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator (0-2) are enabled"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_UP_EN` field.</div>"]
    #[inline(always)]
    pub fn op_up_en(&mut self, n: u8) -> OP_UP_EN_W<'_, UPDATE_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_UP_EN_W::new(self, n * 2 + 2)
    }
    #[doc = "Bit 2 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 0 are enabled"]
    #[inline(always)]
    pub fn op0_up_en(&mut self) -> OP_UP_EN_W<'_, UPDATE_CFG_SPEC> {
        OP_UP_EN_W::new(self, 2)
    }
    #[doc = "Bit 4 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 1 are enabled"]
    #[inline(always)]
    pub fn op1_up_en(&mut self) -> OP_UP_EN_W<'_, UPDATE_CFG_SPEC> {
        OP_UP_EN_W::new(self, 4)
    }
    #[doc = "Bit 6 - When set and PWM_GLOBAL_UP_EN is set, update of active registers in PWM operator 2 are enabled"]
    #[inline(always)]
    pub fn op2_up_en(&mut self) -> OP_UP_EN_W<'_, UPDATE_CFG_SPEC> {
        OP_UP_EN_W::new(self, 6)
    }
    #[doc = "A toggle (software invert its value) will trigger a forced update of active registers in PWM operator (0-2)"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `OP0_FORCE_UP` field.</div>"]
    #[inline(always)]
    pub fn op_force_up(&mut self, n: u8) -> OP_FORCE_UP_W<'_, UPDATE_CFG_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 3][n as usize];
        OP_FORCE_UP_W::new(self, n * 2 + 3)
    }
    #[doc = "Bit 3 - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator 0"]
    #[inline(always)]
    pub fn op0_force_up(&mut self) -> OP_FORCE_UP_W<'_, UPDATE_CFG_SPEC> {
        OP_FORCE_UP_W::new(self, 3)
    }
    #[doc = "Bit 5 - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator 1"]
    #[inline(always)]
    pub fn op1_force_up(&mut self) -> OP_FORCE_UP_W<'_, UPDATE_CFG_SPEC> {
        OP_FORCE_UP_W::new(self, 5)
    }
    #[doc = "Bit 7 - A toggle (software invert its value) will trigger a forced update of active registers in PWM operator 2"]
    #[inline(always)]
    pub fn op2_force_up(&mut self) -> OP_FORCE_UP_W<'_, UPDATE_CFG_SPEC> {
        OP_FORCE_UP_W::new(self, 7)
    }
}
#[doc = "Enable update.\n\nYou can [`read`](crate::Reg::read) this register and get [`update_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`update_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UPDATE_CFG_SPEC;
impl crate::RegisterSpec for UPDATE_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`update_cfg::R`](R) reader structure"]
impl crate::Readable for UPDATE_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`update_cfg::W`](W) writer structure"]
impl crate::Writable for UPDATE_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets UPDATE_CFG to value 0x55"]
impl crate::Resettable for UPDATE_CFG_SPEC {
    const RESET_VALUE: u32 = 0x55;
}
