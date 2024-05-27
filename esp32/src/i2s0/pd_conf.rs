#[doc = "Register `PD_CONF` reader"]
pub type R = crate::R<PD_CONF_SPEC>;
#[doc = "Register `PD_CONF` writer"]
pub type W = crate::W<PD_CONF_SPEC>;
#[doc = "Field `FIFO_FORCE_PD` reader - "]
pub type FIFO_FORCE_PD_R = crate::BitReader;
#[doc = "Field `FIFO_FORCE_PD` writer - "]
pub type FIFO_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `FIFO_FORCE_PU` reader - "]
pub type FIFO_FORCE_PU_R = crate::BitReader;
#[doc = "Field `FIFO_FORCE_PU` writer - "]
pub type FIFO_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLC_MEM_FORCE_PD` reader - "]
pub type PLC_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `PLC_MEM_FORCE_PD` writer - "]
pub type PLC_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `PLC_MEM_FORCE_PU` reader - "]
pub type PLC_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `PLC_MEM_FORCE_PU` writer - "]
pub type PLC_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn fifo_force_pd(&self) -> FIFO_FORCE_PD_R {
        FIFO_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn fifo_force_pu(&self) -> FIFO_FORCE_PU_R {
        FIFO_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn plc_mem_force_pd(&self) -> PLC_MEM_FORCE_PD_R {
        PLC_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn plc_mem_force_pu(&self) -> PLC_MEM_FORCE_PU_R {
        PLC_MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PD_CONF")
            .field("fifo_force_pd", &self.fifo_force_pd())
            .field("fifo_force_pu", &self.fifo_force_pu())
            .field("plc_mem_force_pd", &self.plc_mem_force_pd())
            .field("plc_mem_force_pu", &self.plc_mem_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_force_pd(&mut self) -> FIFO_FORCE_PD_W<PD_CONF_SPEC> {
        FIFO_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    #[must_use]
    pub fn fifo_force_pu(&mut self) -> FIFO_FORCE_PU_W<PD_CONF_SPEC> {
        FIFO_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    #[must_use]
    pub fn plc_mem_force_pd(&mut self) -> PLC_MEM_FORCE_PD_W<PD_CONF_SPEC> {
        PLC_MEM_FORCE_PD_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    #[must_use]
    pub fn plc_mem_force_pu(&mut self) -> PLC_MEM_FORCE_PU_W<PD_CONF_SPEC> {
        PLC_MEM_FORCE_PU_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`pd_conf::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`pd_conf::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PD_CONF_SPEC;
impl crate::RegisterSpec for PD_CONF_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pd_conf::R`](R) reader structure"]
impl crate::Readable for PD_CONF_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pd_conf::W`](W) writer structure"]
impl crate::Writable for PD_CONF_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PD_CONF to value 0x0a"]
impl crate::Resettable for PD_CONF_SPEC {
    const RESET_VALUE: u32 = 0x0a;
}
