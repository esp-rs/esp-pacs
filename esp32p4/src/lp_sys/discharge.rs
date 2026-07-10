#[doc = "Register `DISCHARGE` reader"]
pub type R = crate::R<DISCHARGE_SPEC>;
#[doc = "Register `DISCHARGE` writer"]
pub type W = crate::W<DISCHARGE_SPEC>;
#[doc = "Field `LDO_FLASH_DISCHARGE` reader - Set this bit to discharge ldo flash."]
pub type LDO_FLASH_DISCHARGE_R = crate::BitReader;
#[doc = "Field `LDO_FLASH_DISCHARGE` writer - Set this bit to discharge ldo flash."]
pub type LDO_FLASH_DISCHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PUF_MEM_ISO_EN` reader - Set this bit to discharge lp puf mem."]
pub type LP_PUF_MEM_ISO_EN_R = crate::BitReader;
#[doc = "Field `LP_PUF_MEM_ISO_EN` writer - Set this bit to discharge lp puf mem."]
pub type LP_PUF_MEM_ISO_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PUF_MEM_XPD` reader - Set this bit to discharge lp puf mem."]
pub type LP_PUF_MEM_XPD_R = crate::BitReader;
#[doc = "Field `LP_PUF_MEM_XPD` writer - Set this bit to discharge lp puf mem."]
pub type LP_PUF_MEM_XPD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LP_PUF_MEM_DISCHARGE` reader - Set this bit to discharge lp puf mem."]
pub type LP_PUF_MEM_DISCHARGE_R = crate::BitReader;
#[doc = "Field `LP_PUF_MEM_DISCHARGE` writer - Set this bit to discharge lp puf mem."]
pub type LP_PUF_MEM_DISCHARGE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - Set this bit to discharge ldo flash."]
    #[inline(always)]
    pub fn ldo_flash_discharge(&self) -> LDO_FLASH_DISCHARGE_R {
        LDO_FLASH_DISCHARGE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Set this bit to discharge lp puf mem."]
    #[inline(always)]
    pub fn lp_puf_mem_iso_en(&self) -> LP_PUF_MEM_ISO_EN_R {
        LP_PUF_MEM_ISO_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Set this bit to discharge lp puf mem."]
    #[inline(always)]
    pub fn lp_puf_mem_xpd(&self) -> LP_PUF_MEM_XPD_R {
        LP_PUF_MEM_XPD_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Set this bit to discharge lp puf mem."]
    #[inline(always)]
    pub fn lp_puf_mem_discharge(&self) -> LP_PUF_MEM_DISCHARGE_R {
        LP_PUF_MEM_DISCHARGE_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("DISCHARGE")
            .field("ldo_flash_discharge", &self.ldo_flash_discharge())
            .field("lp_puf_mem_iso_en", &self.lp_puf_mem_iso_en())
            .field("lp_puf_mem_xpd", &self.lp_puf_mem_xpd())
            .field("lp_puf_mem_discharge", &self.lp_puf_mem_discharge())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to discharge ldo flash."]
    #[inline(always)]
    pub fn ldo_flash_discharge(&mut self) -> LDO_FLASH_DISCHARGE_W<'_, DISCHARGE_SPEC> {
        LDO_FLASH_DISCHARGE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to discharge lp puf mem."]
    #[inline(always)]
    pub fn lp_puf_mem_iso_en(&mut self) -> LP_PUF_MEM_ISO_EN_W<'_, DISCHARGE_SPEC> {
        LP_PUF_MEM_ISO_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to discharge lp puf mem."]
    #[inline(always)]
    pub fn lp_puf_mem_xpd(&mut self) -> LP_PUF_MEM_XPD_W<'_, DISCHARGE_SPEC> {
        LP_PUF_MEM_XPD_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to discharge lp puf mem."]
    #[inline(always)]
    pub fn lp_puf_mem_discharge(&mut self) -> LP_PUF_MEM_DISCHARGE_W<'_, DISCHARGE_SPEC> {
        LP_PUF_MEM_DISCHARGE_W::new(self, 3)
    }
}
#[doc = "pufmem / ldo flash power discharge control\n\nYou can [`read`](crate::Reg::read) this register and get [`discharge::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`discharge::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct DISCHARGE_SPEC;
impl crate::RegisterSpec for DISCHARGE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`discharge::R`](R) reader structure"]
impl crate::Readable for DISCHARGE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`discharge::W`](W) writer structure"]
impl crate::Writable for DISCHARGE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets DISCHARGE to value 0x04"]
impl crate::Resettable for DISCHARGE_SPEC {
    const RESET_VALUE: u32 = 0x04;
}
