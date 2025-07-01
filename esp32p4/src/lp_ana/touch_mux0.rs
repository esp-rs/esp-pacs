#[doc = "Register `TOUCH_MUX0` reader"]
pub type R = crate::R<TOUCH_MUX0_SPEC>;
#[doc = "Register `TOUCH_MUX0` writer"]
pub type W = crate::W<TOUCH_MUX0_SPEC>;
#[doc = "Field `TOUCH_DATA_SEL` reader - need_des"]
pub type TOUCH_DATA_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_DATA_SEL` writer - need_des"]
pub type TOUCH_DATA_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_FREQ_SEL` reader - need_des"]
pub type TOUCH_FREQ_SEL_R = crate::FieldReader;
#[doc = "Field `TOUCH_FREQ_SEL` writer - need_des"]
pub type TOUCH_FREQ_SEL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TOUCH_BUFSEL` reader - need_des"]
pub type TOUCH_BUFSEL_R = crate::FieldReader<u16>;
#[doc = "Field `TOUCH_BUFSEL` writer - need_des"]
pub type TOUCH_BUFSEL_W<'a, REG> = crate::FieldWriter<'a, REG, 15, u16>;
#[doc = "Field `TOUCH_DONE_EN` reader - need_des"]
pub type TOUCH_DONE_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE_EN` writer - need_des"]
pub type TOUCH_DONE_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_DONE_FORCE` reader - need_des"]
pub type TOUCH_DONE_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_DONE_FORCE` writer - need_des"]
pub type TOUCH_DONE_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_FSM_EN` reader - need_des"]
pub type TOUCH_FSM_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_FSM_EN` writer - need_des"]
pub type TOUCH_FSM_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_EN` reader - need_des"]
pub type TOUCH_START_EN_R = crate::BitReader;
#[doc = "Field `TOUCH_START_EN` writer - need_des"]
pub type TOUCH_START_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TOUCH_START_FORCE` reader - need_des"]
pub type TOUCH_START_FORCE_R = crate::BitReader;
#[doc = "Field `TOUCH_START_FORCE` writer - need_des"]
pub type TOUCH_START_FORCE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 8:9 - need_des"]
    #[inline(always)]
    pub fn touch_data_sel(&self) -> TOUCH_DATA_SEL_R {
        TOUCH_DATA_SEL_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - need_des"]
    #[inline(always)]
    pub fn touch_freq_sel(&self) -> TOUCH_FREQ_SEL_R {
        TOUCH_FREQ_SEL_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:26 - need_des"]
    #[inline(always)]
    pub fn touch_bufsel(&self) -> TOUCH_BUFSEL_R {
        TOUCH_BUFSEL_R::new(((self.bits >> 12) & 0x7fff) as u16)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn touch_done_en(&self) -> TOUCH_DONE_EN_R {
        TOUCH_DONE_EN_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn touch_done_force(&self) -> TOUCH_DONE_FORCE_R {
        TOUCH_DONE_FORCE_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_fsm_en(&self) -> TOUCH_FSM_EN_R {
        TOUCH_FSM_EN_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn touch_start_en(&self) -> TOUCH_START_EN_R {
        TOUCH_START_EN_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn touch_start_force(&self) -> TOUCH_START_FORCE_R {
        TOUCH_START_FORCE_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("TOUCH_MUX0")
            .field("touch_data_sel", &self.touch_data_sel())
            .field("touch_freq_sel", &self.touch_freq_sel())
            .field("touch_bufsel", &self.touch_bufsel())
            .field("touch_done_en", &self.touch_done_en())
            .field("touch_done_force", &self.touch_done_force())
            .field("touch_fsm_en", &self.touch_fsm_en())
            .field("touch_start_en", &self.touch_start_en())
            .field("touch_start_force", &self.touch_start_force())
            .finish()
    }
}
impl W {
    #[doc = "Bits 8:9 - need_des"]
    #[inline(always)]
    pub fn touch_data_sel(&mut self) -> TOUCH_DATA_SEL_W<TOUCH_MUX0_SPEC> {
        TOUCH_DATA_SEL_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - need_des"]
    #[inline(always)]
    pub fn touch_freq_sel(&mut self) -> TOUCH_FREQ_SEL_W<TOUCH_MUX0_SPEC> {
        TOUCH_FREQ_SEL_W::new(self, 10)
    }
    #[doc = "Bits 12:26 - need_des"]
    #[inline(always)]
    pub fn touch_bufsel(&mut self) -> TOUCH_BUFSEL_W<TOUCH_MUX0_SPEC> {
        TOUCH_BUFSEL_W::new(self, 12)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn touch_done_en(&mut self) -> TOUCH_DONE_EN_W<TOUCH_MUX0_SPEC> {
        TOUCH_DONE_EN_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn touch_done_force(&mut self) -> TOUCH_DONE_FORCE_W<TOUCH_MUX0_SPEC> {
        TOUCH_DONE_FORCE_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn touch_fsm_en(&mut self) -> TOUCH_FSM_EN_W<TOUCH_MUX0_SPEC> {
        TOUCH_FSM_EN_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn touch_start_en(&mut self) -> TOUCH_START_EN_W<TOUCH_MUX0_SPEC> {
        TOUCH_START_EN_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn touch_start_force(&mut self) -> TOUCH_START_FORCE_W<TOUCH_MUX0_SPEC> {
        TOUCH_START_FORCE_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`touch_mux0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`touch_mux0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct TOUCH_MUX0_SPEC;
impl crate::RegisterSpec for TOUCH_MUX0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`touch_mux0::R`](R) reader structure"]
impl crate::Readable for TOUCH_MUX0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`touch_mux0::W`](W) writer structure"]
impl crate::Writable for TOUCH_MUX0_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets TOUCH_MUX0 to value 0x2000_0000"]
impl crate::Resettable for TOUCH_MUX0_SPEC {
    const RESET_VALUE: u32 = 0x2000_0000;
}
