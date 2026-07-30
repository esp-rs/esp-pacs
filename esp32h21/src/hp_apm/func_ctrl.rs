#[doc = "Register `FUNC_CTRL` reader"]
pub type R = crate::R<FUNC_CTRL_SPEC>;
#[doc = "Register `FUNC_CTRL` writer"]
pub type W = crate::W<FUNC_CTRL_SPEC>;
#[doc = "Field `M0_PMS_FUNC_EN` reader - PMS M0 function enable"]
pub type M0_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M0_PMS_FUNC_EN` writer - PMS M0 function enable"]
pub type M0_PMS_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M1_PMS_FUNC_EN` reader - PMS M1 function enable"]
pub type M1_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M1_PMS_FUNC_EN` writer - PMS M1 function enable"]
pub type M1_PMS_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M2_PMS_FUNC_EN` reader - PMS M2 function enable"]
pub type M2_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M2_PMS_FUNC_EN` writer - PMS M2 function enable"]
pub type M2_PMS_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `M3_PMS_FUNC_EN` reader - PMS M3 function enable"]
pub type M3_PMS_FUNC_EN_R = crate::BitReader;
#[doc = "Field `M3_PMS_FUNC_EN` writer - PMS M3 function enable"]
pub type M3_PMS_FUNC_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_pms_func_en(&self) -> M0_PMS_FUNC_EN_R {
        M0_PMS_FUNC_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_pms_func_en(&self) -> M1_PMS_FUNC_EN_R {
        M1_PMS_FUNC_EN_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - PMS M2 function enable"]
    #[inline(always)]
    pub fn m2_pms_func_en(&self) -> M2_PMS_FUNC_EN_R {
        M2_PMS_FUNC_EN_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - PMS M3 function enable"]
    #[inline(always)]
    pub fn m3_pms_func_en(&self) -> M3_PMS_FUNC_EN_R {
        M3_PMS_FUNC_EN_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("FUNC_CTRL")
            .field("m0_pms_func_en", &self.m0_pms_func_en())
            .field("m1_pms_func_en", &self.m1_pms_func_en())
            .field("m2_pms_func_en", &self.m2_pms_func_en())
            .field("m3_pms_func_en", &self.m3_pms_func_en())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - PMS M0 function enable"]
    #[inline(always)]
    pub fn m0_pms_func_en(&mut self) -> M0_PMS_FUNC_EN_W<'_, FUNC_CTRL_SPEC> {
        M0_PMS_FUNC_EN_W::new(self, 0)
    }
    #[doc = "Bit 1 - PMS M1 function enable"]
    #[inline(always)]
    pub fn m1_pms_func_en(&mut self) -> M1_PMS_FUNC_EN_W<'_, FUNC_CTRL_SPEC> {
        M1_PMS_FUNC_EN_W::new(self, 1)
    }
    #[doc = "Bit 2 - PMS M2 function enable"]
    #[inline(always)]
    pub fn m2_pms_func_en(&mut self) -> M2_PMS_FUNC_EN_W<'_, FUNC_CTRL_SPEC> {
        M2_PMS_FUNC_EN_W::new(self, 2)
    }
    #[doc = "Bit 3 - PMS M3 function enable"]
    #[inline(always)]
    pub fn m3_pms_func_en(&mut self) -> M3_PMS_FUNC_EN_W<'_, FUNC_CTRL_SPEC> {
        M3_PMS_FUNC_EN_W::new(self, 3)
    }
}
#[doc = "PMS function control register\n\nYou can [`read`](crate::Reg::read) this register and get [`func_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`func_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct FUNC_CTRL_SPEC;
impl crate::RegisterSpec for FUNC_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`func_ctrl::R`](R) reader structure"]
impl crate::Readable for FUNC_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`func_ctrl::W`](W) writer structure"]
impl crate::Writable for FUNC_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets FUNC_CTRL to value 0x0f"]
impl crate::Resettable for FUNC_CTRL_SPEC {
    const RESET_VALUE: u32 = 0x0f;
}
