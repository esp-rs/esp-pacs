#[doc = "Register `MEM_CTRL` reader"]
pub type R = crate::R<MEM_CTRL_SPEC>;
#[doc = "Register `MEM_CTRL` writer"]
pub type W = crate::W<MEM_CTRL_SPEC>;
#[doc = "Field `LP_MEM_FORCE_PD` reader - force off lp memory"]
pub type LP_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `LP_MEM_FORCE_PD` writer - force off lp memory"]
pub type LP_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_MEM_FORCE_PU` reader - force on lp memory"]
pub type LP_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `LP_MEM_FORCE_PU` writer - force on lp memory"]
pub type LP_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUK_MEM_FORCE_PD` reader - force off huk memory"]
pub type HUK_MEM_FORCE_PD_R = crate::BitReader;
#[doc = "Field `HUK_MEM_FORCE_PD` writer - force off huk memory"]
pub type HUK_MEM_FORCE_PD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUK_MEM_FORCE_PU` reader - force on huk memory"]
pub type HUK_MEM_FORCE_PU_R = crate::BitReader;
#[doc = "Field `HUK_MEM_FORCE_PU` writer - force on huk memory"]
pub type HUK_MEM_FORCE_PU_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - force off lp memory"]
    #[inline(always)]
    pub fn lp_mem_force_pd(&self) -> LP_MEM_FORCE_PD_R {
        LP_MEM_FORCE_PD_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - force on lp memory"]
    #[inline(always)]
    pub fn lp_mem_force_pu(&self) -> LP_MEM_FORCE_PU_R {
        LP_MEM_FORCE_PU_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - force off huk memory"]
    #[inline(always)]
    pub fn huk_mem_force_pd(&self) -> HUK_MEM_FORCE_PD_R {
        HUK_MEM_FORCE_PD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - force on huk memory"]
    #[inline(always)]
    pub fn huk_mem_force_pu(&self) -> HUK_MEM_FORCE_PU_R {
        HUK_MEM_FORCE_PU_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CTRL")
            .field("lp_mem_force_pd", &self.lp_mem_force_pd())
            .field("lp_mem_force_pu", &self.lp_mem_force_pu())
            .field("huk_mem_force_pd", &self.huk_mem_force_pd())
            .field("huk_mem_force_pu", &self.huk_mem_force_pu())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - force off lp memory"]
    #[inline(always)]
    pub fn lp_mem_force_pd(&mut self) -> LP_MEM_FORCE_PD_W<MEM_CTRL_SPEC> {
        LP_MEM_FORCE_PD_W::new(self, 0)
    }
    #[doc = "Bit 1 - force on lp memory"]
    #[inline(always)]
    pub fn lp_mem_force_pu(&mut self) -> LP_MEM_FORCE_PU_W<MEM_CTRL_SPEC> {
        LP_MEM_FORCE_PU_W::new(self, 1)
    }
    #[doc = "Bit 2 - force off huk memory"]
    #[inline(always)]
    pub fn huk_mem_force_pd(&mut self) -> HUK_MEM_FORCE_PD_W<MEM_CTRL_SPEC> {
        HUK_MEM_FORCE_PD_W::new(self, 2)
    }
    #[doc = "Bit 3 - force on huk memory"]
    #[inline(always)]
    pub fn huk_mem_force_pu(&mut self) -> HUK_MEM_FORCE_PU_W<MEM_CTRL_SPEC> {
        HUK_MEM_FORCE_PU_W::new(self, 3)
    }
}
#[doc = "configure rmemory power in lp system register\n\nYou can [`read`](crate::Reg::read) this register and get [`mem_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`mem_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MEM_CTRL_SPEC;
impl crate::RegisterSpec for MEM_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`mem_ctrl::R`](R) reader structure"]
impl crate::Readable for MEM_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`mem_ctrl::W`](W) writer structure"]
impl crate::Writable for MEM_CTRL_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets MEM_CTRL to value 0x06"]
impl crate::Resettable for MEM_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x06;
}
