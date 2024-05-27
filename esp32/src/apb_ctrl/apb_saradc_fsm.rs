#[doc = "Register `APB_SARADC_FSM` reader"]
pub type R = crate::R<APB_SARADC_FSM_SPEC>;
#[doc = "Register `APB_SARADC_FSM` writer"]
pub type W = crate::W<APB_SARADC_FSM_SPEC>;
#[doc = "Field `SARADC_RSTB_WAIT` reader - "]
pub type SARADC_RSTB_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_RSTB_WAIT` writer - "]
pub type SARADC_RSTB_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SARADC_STANDBY_WAIT` reader - "]
pub type SARADC_STANDBY_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_STANDBY_WAIT` writer - "]
pub type SARADC_STANDBY_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SARADC_START_WAIT` reader - "]
pub type SARADC_START_WAIT_R = crate::FieldReader;
#[doc = "Field `SARADC_START_WAIT` writer - "]
pub type SARADC_START_WAIT_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SARADC_SAMPLE_CYCLE` reader - sample cycles"]
pub type SARADC_SAMPLE_CYCLE_R = crate::FieldReader;
#[doc = "Field `SARADC_SAMPLE_CYCLE` writer - sample cycles"]
pub type SARADC_SAMPLE_CYCLE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn saradc_rstb_wait(&self) -> SARADC_RSTB_WAIT_R {
        SARADC_RSTB_WAIT_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn saradc_standby_wait(&self) -> SARADC_STANDBY_WAIT_R {
        SARADC_STANDBY_WAIT_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    pub fn saradc_start_wait(&self) -> SARADC_START_WAIT_R {
        SARADC_START_WAIT_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    pub fn saradc_sample_cycle(&self) -> SARADC_SAMPLE_CYCLE_R {
        SARADC_SAMPLE_CYCLE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("APB_SARADC_FSM")
            .field("saradc_rstb_wait", &self.saradc_rstb_wait())
            .field("saradc_standby_wait", &self.saradc_standby_wait())
            .field("saradc_start_wait", &self.saradc_start_wait())
            .field("saradc_sample_cycle", &self.saradc_sample_cycle())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_rstb_wait(&mut self) -> SARADC_RSTB_WAIT_W<APB_SARADC_FSM_SPEC> {
        SARADC_RSTB_WAIT_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_standby_wait(&mut self) -> SARADC_STANDBY_WAIT_W<APB_SARADC_FSM_SPEC> {
        SARADC_STANDBY_WAIT_W::new(self, 8)
    }
    #[doc = "Bits 16:23"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_start_wait(&mut self) -> SARADC_START_WAIT_W<APB_SARADC_FSM_SPEC> {
        SARADC_START_WAIT_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - sample cycles"]
    #[inline(always)]
    #[must_use]
    pub fn saradc_sample_cycle(&mut self) -> SARADC_SAMPLE_CYCLE_W<APB_SARADC_FSM_SPEC> {
        SARADC_SAMPLE_CYCLE_W::new(self, 24)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`apb_saradc_fsm::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`apb_saradc_fsm::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct APB_SARADC_FSM_SPEC;
impl crate::RegisterSpec for APB_SARADC_FSM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`apb_saradc_fsm::R`](R) reader structure"]
impl crate::Readable for APB_SARADC_FSM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`apb_saradc_fsm::W`](W) writer structure"]
impl crate::Writable for APB_SARADC_FSM_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets APB_SARADC_FSM to value 0x0208_ff08"]
impl crate::Resettable for APB_SARADC_FSM_SPEC {
    const RESET_VALUE: u32 = 0x0208_ff08;
}
