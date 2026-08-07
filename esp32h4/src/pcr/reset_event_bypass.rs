#[doc = "Register `RESET_EVENT_BYPASS` reader"]
pub type R = crate::R<RESET_EVENT_BYPASS_SPEC>;
#[doc = "Register `RESET_EVENT_BYPASS` writer"]
pub type W = crate::W<RESET_EVENT_BYPASS_SPEC>;
#[doc = "Field `TEE_APM` reader - This field is used to control reset event relationship for tee_apm. 1: tee_apm will only be reset by power-reset. some reset event will be bypass. 0:tee_apm will not only be reset by power-reset, but also some reset event."]
pub type TEE_APM_R = crate::BitReader;
#[doc = "Field `TEE_APM` writer - This field is used to control reset event relationship for tee_apm. 1: tee_apm will only be reset by power-reset. some reset event will be bypass. 0:tee_apm will not only be reset by power-reset, but also some reset event."]
pub type TEE_APM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PCR` reader - This field is used to control reset event relationship for pcr. 1: pcr will only be reset by power-reset. some reset event will be bypass. 0:pcr will not only be reset by power-reset, but also some reset event."]
pub type PCR_R = crate::BitReader;
#[doc = "Field `PCR` writer - This field is used to control reset event relationship for pcr. 1: pcr will only be reset by power-reset. some reset event will be bypass. 0:pcr will not only be reset by power-reset, but also some reset event."]
pub type PCR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPSYSREG` reader - This field is used to control reset event relationship for hpsysreg. 1: hpsysreg will only be reset by power-reset. some reset event will be bypass. 0:hpsysreg will not only be reset by power-reset, but also some reset event."]
pub type HPSYSREG_R = crate::BitReader;
#[doc = "Field `HPSYSREG` writer - This field is used to control reset event relationship for hpsysreg. 1: hpsysreg will only be reset by power-reset. some reset event will be bypass. 0:hpsysreg will not only be reset by power-reset, but also some reset event."]
pub type HPSYSREG_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IOMUX` reader - This field is used to control reset event relationship for iomux. 1: iomux will only be reset by power-reset. some reset event will be bypass. 0:iomux will not only be reset by power-reset, but also some reset event."]
pub type IOMUX_R = crate::BitReader;
#[doc = "Field `IOMUX` writer - This field is used to control reset event relationship for iomux. 1: iomux will only be reset by power-reset. some reset event will be bypass. 0:iomux will not only be reset by power-reset, but also some reset event."]
pub type IOMUX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMTX0` reader - This field is used to control reset event relationship for intmtx0. 1: intmtx0 will only be reset by power-reset. some reset event will be bypass. 0:intmtx0 will not only be reset by power-reset, but also some reset event."]
pub type INTMTX0_R = crate::BitReader;
#[doc = "Field `INTMTX0` writer - This field is used to control reset event relationship for intmtx0. 1: intmtx0 will only be reset by power-reset. some reset event will be bypass. 0:intmtx0 will not only be reset by power-reset, but also some reset event."]
pub type INTMTX0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INTMTX1` reader - This field is used to control reset event relationship for intmtx1. 1: intmtx1 will only be reset by power-reset. some reset event will be bypass. 0:intmtx1 will not only be reset by power-reset, but also some reset event."]
pub type INTMTX1_R = crate::BitReader;
#[doc = "Field `INTMTX1` writer - This field is used to control reset event relationship for intmtx1. 1: intmtx1 will only be reset by power-reset. some reset event will be bypass. 0:intmtx1 will not only be reset by power-reset, but also some reset event."]
pub type INTMTX1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MODEM` reader - This field is used to control reset event relationship for modem. 1: modem will only be reset by power-reset. some reset event will be bypass. 0:modem will not only be reset by power-reset, but also some reset event."]
pub type MODEM_R = crate::BitReader;
#[doc = "Field `MODEM` writer - This field is used to control reset event relationship for modem. 1: modem will only be reset by power-reset. some reset event will be bypass. 0:modem will not only be reset by power-reset, but also some reset event."]
pub type MODEM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BUS_MODEM` reader - This field is used to control reset event relationship for bus_modem. 1: bus_modem will only be reset by power-reset. some reset event will be bypass. 0:bus_modem will not only be reset by power-reset, but also some reset event."]
pub type BUS_MODEM_R = crate::BitReader;
#[doc = "Field `BUS_MODEM` writer - This field is used to control reset event relationship for bus_modem. 1: bus_modem will only be reset by power-reset. some reset event will be bypass. 0:bus_modem will not only be reset by power-reset, but also some reset event."]
pub type BUS_MODEM_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This field is used to control reset event relationship for tee_apm. 1: tee_apm will only be reset by power-reset. some reset event will be bypass. 0:tee_apm will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn tee_apm(&self) -> TEE_APM_R {
        TEE_APM_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This field is used to control reset event relationship for pcr. 1: pcr will only be reset by power-reset. some reset event will be bypass. 0:pcr will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn pcr(&self) -> PCR_R {
        PCR_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This field is used to control reset event relationship for hpsysreg. 1: hpsysreg will only be reset by power-reset. some reset event will be bypass. 0:hpsysreg will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn hpsysreg(&self) -> HPSYSREG_R {
        HPSYSREG_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This field is used to control reset event relationship for iomux. 1: iomux will only be reset by power-reset. some reset event will be bypass. 0:iomux will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn iomux(&self) -> IOMUX_R {
        IOMUX_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This field is used to control reset event relationship for intmtx0. 1: intmtx0 will only be reset by power-reset. some reset event will be bypass. 0:intmtx0 will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn intmtx0(&self) -> INTMTX0_R {
        INTMTX0_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This field is used to control reset event relationship for intmtx1. 1: intmtx1 will only be reset by power-reset. some reset event will be bypass. 0:intmtx1 will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn intmtx1(&self) -> INTMTX1_R {
        INTMTX1_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This field is used to control reset event relationship for modem. 1: modem will only be reset by power-reset. some reset event will be bypass. 0:modem will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn modem(&self) -> MODEM_R {
        MODEM_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This field is used to control reset event relationship for bus_modem. 1: bus_modem will only be reset by power-reset. some reset event will be bypass. 0:bus_modem will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn bus_modem(&self) -> BUS_MODEM_R {
        BUS_MODEM_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RESET_EVENT_BYPASS")
            .field("tee_apm", &self.tee_apm())
            .field("pcr", &self.pcr())
            .field("hpsysreg", &self.hpsysreg())
            .field("iomux", &self.iomux())
            .field("intmtx0", &self.intmtx0())
            .field("intmtx1", &self.intmtx1())
            .field("modem", &self.modem())
            .field("bus_modem", &self.bus_modem())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This field is used to control reset event relationship for tee_apm. 1: tee_apm will only be reset by power-reset. some reset event will be bypass. 0:tee_apm will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn tee_apm(&mut self) -> TEE_APM_W<'_, RESET_EVENT_BYPASS_SPEC> {
        TEE_APM_W::new(self, 0)
    }
    #[doc = "Bit 1 - This field is used to control reset event relationship for pcr. 1: pcr will only be reset by power-reset. some reset event will be bypass. 0:pcr will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn pcr(&mut self) -> PCR_W<'_, RESET_EVENT_BYPASS_SPEC> {
        PCR_W::new(self, 1)
    }
    #[doc = "Bit 2 - This field is used to control reset event relationship for hpsysreg. 1: hpsysreg will only be reset by power-reset. some reset event will be bypass. 0:hpsysreg will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn hpsysreg(&mut self) -> HPSYSREG_W<'_, RESET_EVENT_BYPASS_SPEC> {
        HPSYSREG_W::new(self, 2)
    }
    #[doc = "Bit 3 - This field is used to control reset event relationship for iomux. 1: iomux will only be reset by power-reset. some reset event will be bypass. 0:iomux will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn iomux(&mut self) -> IOMUX_W<'_, RESET_EVENT_BYPASS_SPEC> {
        IOMUX_W::new(self, 3)
    }
    #[doc = "Bit 4 - This field is used to control reset event relationship for intmtx0. 1: intmtx0 will only be reset by power-reset. some reset event will be bypass. 0:intmtx0 will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn intmtx0(&mut self) -> INTMTX0_W<'_, RESET_EVENT_BYPASS_SPEC> {
        INTMTX0_W::new(self, 4)
    }
    #[doc = "Bit 5 - This field is used to control reset event relationship for intmtx1. 1: intmtx1 will only be reset by power-reset. some reset event will be bypass. 0:intmtx1 will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn intmtx1(&mut self) -> INTMTX1_W<'_, RESET_EVENT_BYPASS_SPEC> {
        INTMTX1_W::new(self, 5)
    }
    #[doc = "Bit 6 - This field is used to control reset event relationship for modem. 1: modem will only be reset by power-reset. some reset event will be bypass. 0:modem will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn modem(&mut self) -> MODEM_W<'_, RESET_EVENT_BYPASS_SPEC> {
        MODEM_W::new(self, 6)
    }
    #[doc = "Bit 7 - This field is used to control reset event relationship for bus_modem. 1: bus_modem will only be reset by power-reset. some reset event will be bypass. 0:bus_modem will not only be reset by power-reset, but also some reset event."]
    #[inline(always)]
    pub fn bus_modem(&mut self) -> BUS_MODEM_W<'_, RESET_EVENT_BYPASS_SPEC> {
        BUS_MODEM_W::new(self, 7)
    }
}
#[doc = "reset event bypass backdoor configuration register\n\nYou can [`read`](crate::Reg::read) this register and get [`reset_event_bypass::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`reset_event_bypass::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RESET_EVENT_BYPASS_SPEC;
impl crate::RegisterSpec for RESET_EVENT_BYPASS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`reset_event_bypass::R`](R) reader structure"]
impl crate::Readable for RESET_EVENT_BYPASS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`reset_event_bypass::W`](W) writer structure"]
impl crate::Writable for RESET_EVENT_BYPASS_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RESET_EVENT_BYPASS to value 0"]
impl crate::Resettable for RESET_EVENT_BYPASS_SPEC {}
