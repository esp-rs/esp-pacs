#[doc = "Register `RMT_PD_CTRL` reader"]
pub type R = crate::R<RMT_PD_CTRL_SPEC>;
#[doc = "Register `RMT_PD_CTRL` writer"]
pub type W = crate::W<RMT_PD_CTRL_SPEC>;
#[doc = "Field `RMT_MEM_FORCE_PU` reader - Set this bit to force power up RMT memory."]
pub type RMT_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `RMT_MEM_FORCE_PU` writer - Set this bit to force power up RMT memory."]
pub type RMT_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RMT_MEM_FORCE_PD` reader - Set this bit to force power down RMT memory."]
pub type RMT_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `RMT_MEM_FORCE_PD` writer - Set this bit to force power down RMT memory."]
pub type RMT_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 1 - Set this bit to force power up RMT memory."]
    #[inline(always)]
    pub fn rmt_mem_force_pu(&self) -> RMT_MEM_FORCE_PU_R {
        RMT_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to force power down RMT memory."]
    #[inline(always)]
    pub fn rmt_mem_force_pd(&self) -> RMT_MEM_FORCE_PD_R {
        RMT_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RMT_PD_CTRL")
            .field("rmt_mem_force_pu", &self.rmt_mem_force_pu())
            .field("rmt_mem_force_pd", &self.rmt_mem_force_pd())
            .finish()
    }
}
impl W {
    #[doc = "Bit 1 - Set this bit to force power up RMT memory."]
    #[inline(always)]
    pub fn rmt_mem_force_pu(&mut self) -> RMT_MEM_FORCE_PU_W<RMT_PD_CTRL_SPEC> {
        RMT_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to force power down RMT memory."]
    #[inline(always)]
    pub fn rmt_mem_force_pd(&mut self) -> RMT_MEM_FORCE_PD_W<RMT_PD_CTRL_SPEC> {
        RMT_MEM_FORCE_PD_W::new(self, 2)
    }
}
#[doc = "RMT power control register\n\nYou can [`read`](crate::Reg::read) this register and get [`rmt_pd_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rmt_pd_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RMT_PD_CTRL_SPEC;
impl crate::RegisterSpec for RMT_PD_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rmt_pd_ctrl::R`](R) reader structure"]
impl crate::Readable for RMT_PD_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rmt_pd_ctrl::W`](W) writer structure"]
impl crate::Writable for RMT_PD_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RMT_PD_CTRL to value 0x04"]
impl crate::Resettable for RMT_PD_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
