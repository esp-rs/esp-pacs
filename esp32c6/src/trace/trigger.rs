#[doc = "Register `TRIGGER` reader"]
pub type R = crate::R<TRIGGER_SPEC>;
#[doc = "Register `TRIGGER` writer"]
pub type W = crate::W<TRIGGER_SPEC>;
#[doc = "Field `ON` writer - 0\\] set 1 start trace."]
pub type ON_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `OFF` writer - set 1 stop trace."]
pub type OFF_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `MEM_LOOP` reader - if this reg is 1, trace will loop wrtie trace_mem. If is 0, when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr"]
pub type MEM_LOOP_R = crate::BitReader;
#[doc = "Field `MEM_LOOP` writer - if this reg is 1, trace will loop wrtie trace_mem. If is 0, when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr"]
pub type MEM_LOOP_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
#[doc = "Field `RESTART_ENA` reader - enable encoder auto-restart, when lost package, the encoder will end, if enable auto-restart, when fifo empty, encoder will restart and send a sync package."]
pub type RESTART_ENA_R = crate::BitReader;
#[doc = "Field `RESTART_ENA` writer - enable encoder auto-restart, when lost package, the encoder will end, if enable auto-restart, when fifo empty, encoder will restart and send a sync package."]
pub type RESTART_ENA_W<'a, REG, const O: u8> = crate::BitWriter<'a, REG, O>;
impl R {
    #[doc = "Bit 2 - if this reg is 1, trace will loop wrtie trace_mem. If is 0, when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr"]
    #[inline(always)]
    pub fn mem_loop(&self) -> MEM_LOOP_R {
        MEM_LOOP_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - enable encoder auto-restart, when lost package, the encoder will end, if enable auto-restart, when fifo empty, encoder will restart and send a sync package."]
    #[inline(always)]
    pub fn restart_ena(&self) -> RESTART_ENA_R {
        RESTART_ENA_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TRIGGER")
            .field("mem_loop", &format_args!("{}", self.mem_loop().bit()))
            .field("restart_ena", &format_args!("{}", self.restart_ena().bit()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<TRIGGER_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
impl W {
    #[doc = "Bit 0 - 0\\] set 1 start trace."]
    #[inline(always)]
    #[must_use]
    pub fn on(&mut self) -> ON_W<TRIGGER_SPEC, 0> {
        ON_W::new(self)
    }
    #[doc = "Bit 1 - set 1 stop trace."]
    #[inline(always)]
    #[must_use]
    pub fn off(&mut self) -> OFF_W<TRIGGER_SPEC, 1> {
        OFF_W::new(self)
    }
    #[doc = "Bit 2 - if this reg is 1, trace will loop wrtie trace_mem. If is 0, when mem_current_addr at mem_end_addr, it will stop at the mem_end_addr"]
    #[inline(always)]
    #[must_use]
    pub fn mem_loop(&mut self) -> MEM_LOOP_W<TRIGGER_SPEC, 2> {
        MEM_LOOP_W::new(self)
    }
    #[doc = "Bit 3 - enable encoder auto-restart, when lost package, the encoder will end, if enable auto-restart, when fifo empty, encoder will restart and send a sync package."]
    #[inline(always)]
    #[must_use]
    pub fn restart_ena(&mut self) -> RESTART_ENA_W<TRIGGER_SPEC, 3> {
        RESTART_ENA_W::new(self)
    }
    #[doc = r" Writes raw bits to the register."]
    #[doc = r""]
    #[doc = r" # Safety"]
    #[doc = r""]
    #[doc = r" Passing incorrect value can cause undefined behaviour. See reference manual"]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.bits = bits;
        self
    }
}
#[doc = "trigger register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`trigger::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`trigger::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TRIGGER_SPEC;
impl crate::RegisterSpec for TRIGGER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`trigger::R`](R) reader structure"]
impl crate::Readable for TRIGGER_SPEC {}
#[doc = "`write(|w| ..)` method takes [`trigger::W`](W) writer structure"]
impl crate::Writable for TRIGGER_SPEC {
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets TRIGGER to value 0x0c"]
impl crate::Resettable for TRIGGER_SPEC {
    const RESET_VALUE: Self::Ux = 0x0c;
}
