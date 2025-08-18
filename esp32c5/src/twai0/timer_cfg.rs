#[doc = "Register `TIMER_CFG` reader"]
pub type R = crate::R<TIMER_CFG_SPEC>;
#[doc = "Register `TIMER_CFG` writer"]
pub type W = crate::W<TIMER_CFG_SPEC>;
#[doc = "Field `TIMER_CE` reader - TWAI FD timer enable register. 1b0: Not enable 1b1: Enable timer"]
pub type TIMER_CE_R = crate::BitReader;
#[doc = "Field `TIMER_CE` writer - TWAI FD timer enable register. 1b0: Not enable 1b1: Enable timer"]
pub type TIMER_CE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_CLR` writer - TWAI FD timer clear register. 1b0: Not enable 1b1: Enable to clear value"]
pub type TIMER_CLR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_SET` writer - TWAI FD timer set register. 1b0: Not enable 1b1: Enable to set value to ld_val."]
pub type TIMER_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_UP_DN` reader - TWAI FD timer up/down count register. 1b0: Count-down 1b1: Count-up"]
pub type TIMER_UP_DN_R = crate::BitReader;
#[doc = "Field `TIMER_UP_DN` writer - TWAI FD timer up/down count register. 1b0: Count-down 1b1: Count-up"]
pub type TIMER_UP_DN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TIMER_STEP` reader - TWAI FD timer count step register, step=reg_timer_step+1"]
pub type TIMER_STEP_R = crate::FieldReader<u16>;
#[doc = "Field `TIMER_STEP` writer - TWAI FD timer count step register, step=reg_timer_step+1"]
pub type TIMER_STEP_W<'a, REG> = crate::FieldWriter<'a, REG, 16, u16>;
impl R {
    #[doc = "Bit 0 - TWAI FD timer enable register. 1b0: Not enable 1b1: Enable timer"]
    #[inline(always)]
    pub fn timer_ce(&self) -> TIMER_CE_R {
        TIMER_CE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 8 - TWAI FD timer up/down count register. 1b0: Count-down 1b1: Count-up"]
    #[inline(always)]
    pub fn timer_up_dn(&self) -> TIMER_UP_DN_R {
        TIMER_UP_DN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bits 16:31 - TWAI FD timer count step register, step=reg_timer_step+1"]
    #[inline(always)]
    pub fn timer_step(&self) -> TIMER_STEP_R {
        TIMER_STEP_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TIMER_CFG")
            .field("timer_ce", &self.timer_ce())
            .field("timer_up_dn", &self.timer_up_dn())
            .field("timer_step", &self.timer_step())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - TWAI FD timer enable register. 1b0: Not enable 1b1: Enable timer"]
    #[inline(always)]
    pub fn timer_ce(&mut self) -> TIMER_CE_W<'_, TIMER_CFG_SPEC> {
        TIMER_CE_W::new(self, 0)
    }
    #[doc = "Bit 1 - TWAI FD timer clear register. 1b0: Not enable 1b1: Enable to clear value"]
    #[inline(always)]
    pub fn timer_clr(&mut self) -> TIMER_CLR_W<'_, TIMER_CFG_SPEC> {
        TIMER_CLR_W::new(self, 1)
    }
    #[doc = "Bit 2 - TWAI FD timer set register. 1b0: Not enable 1b1: Enable to set value to ld_val."]
    #[inline(always)]
    pub fn timer_set(&mut self) -> TIMER_SET_W<'_, TIMER_CFG_SPEC> {
        TIMER_SET_W::new(self, 2)
    }
    #[doc = "Bit 8 - TWAI FD timer up/down count register. 1b0: Count-down 1b1: Count-up"]
    #[inline(always)]
    pub fn timer_up_dn(&mut self) -> TIMER_UP_DN_W<'_, TIMER_CFG_SPEC> {
        TIMER_UP_DN_W::new(self, 8)
    }
    #[doc = "Bits 16:31 - TWAI FD timer count step register, step=reg_timer_step+1"]
    #[inline(always)]
    pub fn timer_step(&mut self) -> TIMER_STEP_W<'_, TIMER_CFG_SPEC> {
        TIMER_STEP_W::new(self, 16)
    }
}
#[doc = "TWAI FD timer configure register.\n\nYou can [`read`](crate::Reg::read) this register and get [`timer_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`timer_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TIMER_CFG_SPEC;
impl crate::RegisterSpec for TIMER_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`timer_cfg::R`](R) reader structure"]
impl crate::Readable for TIMER_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`timer_cfg::W`](W) writer structure"]
impl crate::Writable for TIMER_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TIMER_CFG to value 0x0100"]
impl crate::Resettable for TIMER_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0100;
}
