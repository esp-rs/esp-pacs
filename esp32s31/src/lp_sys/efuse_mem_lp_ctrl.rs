#[doc = "Register `EFUSE_MEM_LP_CTRL` reader"]
pub type R = crate::R<EFUSE_MEM_LP_CTRL_SPEC>;
#[doc = "Register `EFUSE_MEM_LP_CTRL` writer"]
pub type W = crate::W<EFUSE_MEM_LP_CTRL_SPEC>;
#[doc = "Field `EFUSE_MEM_LP_MODE` reader - "]
pub type EFUSE_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `EFUSE_MEM_LP_MODE` writer - "]
pub type EFUSE_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `EFUSE_MEM_LP_EN` reader - "]
pub type EFUSE_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_LP_EN` writer - "]
pub type EFUSE_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EFUSE_MEM_LP_FORCE_CTRL` reader - "]
pub type EFUSE_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `EFUSE_MEM_LP_FORCE_CTRL` writer - "]
pub type EFUSE_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn efuse_mem_lp_mode(&self) -> EFUSE_MEM_LP_MODE_R {
        EFUSE_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_mem_lp_en(&self) -> EFUSE_MEM_LP_EN_R {
        EFUSE_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_mem_lp_force_ctrl(&self) -> EFUSE_MEM_LP_FORCE_CTRL_R {
        EFUSE_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EFUSE_MEM_LP_CTRL")
            .field("efuse_mem_lp_mode", &self.efuse_mem_lp_mode())
            .field("efuse_mem_lp_en", &self.efuse_mem_lp_en())
            .field("efuse_mem_lp_force_ctrl", &self.efuse_mem_lp_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn efuse_mem_lp_mode(&mut self) -> EFUSE_MEM_LP_MODE_W<'_, EFUSE_MEM_LP_CTRL_SPEC> {
        EFUSE_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn efuse_mem_lp_en(&mut self) -> EFUSE_MEM_LP_EN_W<'_, EFUSE_MEM_LP_CTRL_SPEC> {
        EFUSE_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn efuse_mem_lp_force_ctrl(
        &mut self,
    ) -> EFUSE_MEM_LP_FORCE_CTRL_W<'_, EFUSE_MEM_LP_CTRL_SPEC> {
        EFUSE_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`efuse_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`efuse_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EFUSE_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for EFUSE_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`efuse_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for EFUSE_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`efuse_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for EFUSE_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EFUSE_MEM_LP_CTRL to value 0"]
impl crate::Resettable for EFUSE_MEM_LP_CTRL_SPEC {}
