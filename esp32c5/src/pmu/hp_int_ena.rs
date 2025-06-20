#[doc = "Register `HP_INT_ENA` reader"]
pub type R = crate::R<HP_INT_ENA_SPEC>;
#[doc = "Register `HP_INT_ENA` writer"]
pub type W = crate::W<HP_INT_ENA_SPEC>;
#[doc = "Field `LP_CPU_EXC_INT_ENA` reader - need_des"]
pub type LP_CPU_EXC_INT_ENA_R = crate::BitReader;
#[doc = "Field `LP_CPU_EXC_INT_ENA` writer - need_des"]
pub type LP_CPU_EXC_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SDIO_IDLE_INT_ENA` reader - need_des"]
pub type SDIO_IDLE_INT_ENA_R = crate::BitReader;
#[doc = "Field `SDIO_IDLE_INT_ENA` writer - need_des"]
pub type SDIO_IDLE_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SW_INT_ENA` reader - need_des"]
pub type SW_INT_ENA_R = crate::BitReader;
#[doc = "Field `SW_INT_ENA` writer - need_des"]
pub type SW_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_SLEEP_REJECT_INT_ENA` reader - need_des"]
pub type SOC_SLEEP_REJECT_INT_ENA_R = crate::BitReader;
#[doc = "Field `SOC_SLEEP_REJECT_INT_ENA` writer - need_des"]
pub type SOC_SLEEP_REJECT_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SOC_WAKEUP_INT_ENA` reader - need_des"]
pub type SOC_WAKEUP_INT_ENA_R = crate::BitReader;
#[doc = "Field `SOC_WAKEUP_INT_ENA` writer - need_des"]
pub type SOC_WAKEUP_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_int_ena(&self) -> LP_CPU_EXC_INT_ENA_R {
        LP_CPU_EXC_INT_ENA_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&self) -> SDIO_IDLE_INT_ENA_R {
        SDIO_IDLE_INT_ENA_R::new(((self.bits >> 28) & 1) != 0)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw_int_ena(&self) -> SW_INT_ENA_R {
        SW_INT_ENA_R::new(((self.bits >> 29) & 1) != 0)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject_int_ena(&self) -> SOC_SLEEP_REJECT_INT_ENA_R {
        SOC_SLEEP_REJECT_INT_ENA_R::new(((self.bits >> 30) & 1) != 0)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_ena(&self) -> SOC_WAKEUP_INT_ENA_R {
        SOC_WAKEUP_INT_ENA_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HP_INT_ENA")
            .field("lp_cpu_exc_int_ena", &self.lp_cpu_exc_int_ena())
            .field("sdio_idle_int_ena", &self.sdio_idle_int_ena())
            .field("sw_int_ena", &self.sw_int_ena())
            .field("soc_sleep_reject_int_ena", &self.soc_sleep_reject_int_ena())
            .field("soc_wakeup_int_ena", &self.soc_wakeup_int_ena())
            .finish()
    }
}
impl W {
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn lp_cpu_exc_int_ena(&mut self) -> LP_CPU_EXC_INT_ENA_W<HP_INT_ENA_SPEC> {
        LP_CPU_EXC_INT_ENA_W::new(self, 27)
    }
    #[doc = "Bit 28 - need_des"]
    #[inline(always)]
    pub fn sdio_idle_int_ena(&mut self) -> SDIO_IDLE_INT_ENA_W<HP_INT_ENA_SPEC> {
        SDIO_IDLE_INT_ENA_W::new(self, 28)
    }
    #[doc = "Bit 29 - need_des"]
    #[inline(always)]
    pub fn sw_int_ena(&mut self) -> SW_INT_ENA_W<HP_INT_ENA_SPEC> {
        SW_INT_ENA_W::new(self, 29)
    }
    #[doc = "Bit 30 - need_des"]
    #[inline(always)]
    pub fn soc_sleep_reject_int_ena(&mut self) -> SOC_SLEEP_REJECT_INT_ENA_W<HP_INT_ENA_SPEC> {
        SOC_SLEEP_REJECT_INT_ENA_W::new(self, 30)
    }
    #[doc = "Bit 31 - need_des"]
    #[inline(always)]
    pub fn soc_wakeup_int_ena(&mut self) -> SOC_WAKEUP_INT_ENA_W<HP_INT_ENA_SPEC> {
        SOC_WAKEUP_INT_ENA_W::new(self, 31)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`hp_int_ena::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hp_int_ena::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HP_INT_ENA_SPEC;
impl crate::RegisterSpec for HP_INT_ENA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hp_int_ena::R`](R) reader structure"]
impl crate::Readable for HP_INT_ENA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hp_int_ena::W`](W) writer structure"]
impl crate::Writable for HP_INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HP_INT_ENA to value 0"]
impl crate::Resettable for HP_INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
