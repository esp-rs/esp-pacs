#[doc = "Register `HUK_MEM_LP_CTRL` reader"]
pub type R = crate::R<HUK_MEM_LP_CTRL_SPEC>;
#[doc = "Register `HUK_MEM_LP_CTRL` writer"]
pub type W = crate::W<HUK_MEM_LP_CTRL_SPEC>;
#[doc = "Field `HUK_MEM_LP_MODE` reader - "]
pub type HUK_MEM_LP_MODE_R = crate::FieldReader;
#[doc = "Field `HUK_MEM_LP_MODE` writer - "]
pub type HUK_MEM_LP_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `HUK_MEM_LP_EN` reader - "]
pub type HUK_MEM_LP_EN_R = crate::BitReader;
#[doc = "Field `HUK_MEM_LP_EN` writer - "]
pub type HUK_MEM_LP_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HUK_MEM_LP_FORCE_CTRL` reader - "]
pub type HUK_MEM_LP_FORCE_CTRL_R = crate::BitReader;
#[doc = "Field `HUK_MEM_LP_FORCE_CTRL` writer - "]
pub type HUK_MEM_LP_FORCE_CTRL_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn huk_mem_lp_mode(&self) -> HUK_MEM_LP_MODE_R {
        HUK_MEM_LP_MODE_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn huk_mem_lp_en(&self) -> HUK_MEM_LP_EN_R {
        HUK_MEM_LP_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn huk_mem_lp_force_ctrl(&self) -> HUK_MEM_LP_FORCE_CTRL_R {
        HUK_MEM_LP_FORCE_CTRL_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HUK_MEM_LP_CTRL")
            .field("huk_mem_lp_mode", &self.huk_mem_lp_mode())
            .field("huk_mem_lp_en", &self.huk_mem_lp_en())
            .field("huk_mem_lp_force_ctrl", &self.huk_mem_lp_force_ctrl())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn huk_mem_lp_mode(&mut self) -> HUK_MEM_LP_MODE_W<'_, HUK_MEM_LP_CTRL_SPEC> {
        HUK_MEM_LP_MODE_W::new(self, 0)
    }
    #[doc = "Bit 2"]
    #[inline(always)]
    pub fn huk_mem_lp_en(&mut self) -> HUK_MEM_LP_EN_W<'_, HUK_MEM_LP_CTRL_SPEC> {
        HUK_MEM_LP_EN_W::new(self, 2)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn huk_mem_lp_force_ctrl(&mut self) -> HUK_MEM_LP_FORCE_CTRL_W<'_, HUK_MEM_LP_CTRL_SPEC> {
        HUK_MEM_LP_FORCE_CTRL_W::new(self, 3)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`huk_mem_lp_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`huk_mem_lp_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HUK_MEM_LP_CTRL_SPEC;
impl crate::RegisterSpec for HUK_MEM_LP_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`huk_mem_lp_ctrl::R`](R) reader structure"]
impl crate::Readable for HUK_MEM_LP_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`huk_mem_lp_ctrl::W`](W) writer structure"]
impl crate::Writable for HUK_MEM_LP_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets HUK_MEM_LP_CTRL to value 0"]
impl crate::Resettable for HUK_MEM_LP_CTRL_SPEC {}
