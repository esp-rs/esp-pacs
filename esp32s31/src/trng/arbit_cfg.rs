#[doc = "Register `ARBIT_CFG` reader"]
pub type R = crate::R<ARBIT_CFG_SPEC>;
#[doc = "Register `ARBIT_CFG` writer"]
pub type W = crate::W<ARBIT_CFG_SPEC>;
#[doc = "Field `ARBIT_PRIORITY_0` reader - reg_arbit_priority_0"]
pub type ARBIT_PRIORITY_0_R = crate::FieldReader;
#[doc = "Field `ARBIT_PRIORITY_0` writer - reg_arbit_priority_0"]
pub type ARBIT_PRIORITY_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARBIT_PRIORITY_1` reader - reg_arbit_priority_1"]
pub type ARBIT_PRIORITY_1_R = crate::FieldReader;
#[doc = "Field `ARBIT_PRIORITY_1` writer - reg_arbit_priority_1"]
pub type ARBIT_PRIORITY_1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARBIT_PRIORITY_2` reader - reg_arbit_priority_2"]
pub type ARBIT_PRIORITY_2_R = crate::FieldReader;
#[doc = "Field `ARBIT_PRIORITY_2` writer - reg_arbit_priority_2"]
pub type ARBIT_PRIORITY_2_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARBIT_PRIORITY_3` reader - reg_arbit_priority_3"]
pub type ARBIT_PRIORITY_3_R = crate::FieldReader;
#[doc = "Field `ARBIT_PRIORITY_3` writer - reg_arbit_priority_3"]
pub type ARBIT_PRIORITY_3_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ARBIT_EN` reader - reg_arbit_en"]
pub type ARBIT_EN_R = crate::BitReader;
#[doc = "Field `ARBIT_EN` writer - reg_arbit_en"]
pub type ARBIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:3 - reg_arbit_priority_0"]
    #[inline(always)]
    pub fn arbit_priority_0(&self) -> ARBIT_PRIORITY_0_R {
        ARBIT_PRIORITY_0_R::new((self.bits & 0x0f) as u8)
    }
    #[doc = "Bits 4:7 - reg_arbit_priority_1"]
    #[inline(always)]
    pub fn arbit_priority_1(&self) -> ARBIT_PRIORITY_1_R {
        ARBIT_PRIORITY_1_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
    #[doc = "Bits 8:11 - reg_arbit_priority_2"]
    #[inline(always)]
    pub fn arbit_priority_2(&self) -> ARBIT_PRIORITY_2_R {
        ARBIT_PRIORITY_2_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:15 - reg_arbit_priority_3"]
    #[inline(always)]
    pub fn arbit_priority_3(&self) -> ARBIT_PRIORITY_3_R {
        ARBIT_PRIORITY_3_R::new(((self.bits >> 12) & 0x0f) as u8)
    }
    #[doc = "Bit 16 - reg_arbit_en"]
    #[inline(always)]
    pub fn arbit_en(&self) -> ARBIT_EN_R {
        ARBIT_EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("ARBIT_CFG")
            .field("arbit_priority_0", &self.arbit_priority_0())
            .field("arbit_priority_1", &self.arbit_priority_1())
            .field("arbit_priority_2", &self.arbit_priority_2())
            .field("arbit_priority_3", &self.arbit_priority_3())
            .field("arbit_en", &self.arbit_en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:3 - reg_arbit_priority_0"]
    #[inline(always)]
    pub fn arbit_priority_0(&mut self) -> ARBIT_PRIORITY_0_W<'_, ARBIT_CFG_SPEC> {
        ARBIT_PRIORITY_0_W::new(self, 0)
    }
    #[doc = "Bits 4:7 - reg_arbit_priority_1"]
    #[inline(always)]
    pub fn arbit_priority_1(&mut self) -> ARBIT_PRIORITY_1_W<'_, ARBIT_CFG_SPEC> {
        ARBIT_PRIORITY_1_W::new(self, 4)
    }
    #[doc = "Bits 8:11 - reg_arbit_priority_2"]
    #[inline(always)]
    pub fn arbit_priority_2(&mut self) -> ARBIT_PRIORITY_2_W<'_, ARBIT_CFG_SPEC> {
        ARBIT_PRIORITY_2_W::new(self, 8)
    }
    #[doc = "Bits 12:15 - reg_arbit_priority_3"]
    #[inline(always)]
    pub fn arbit_priority_3(&mut self) -> ARBIT_PRIORITY_3_W<'_, ARBIT_CFG_SPEC> {
        ARBIT_PRIORITY_3_W::new(self, 12)
    }
    #[doc = "Bit 16 - reg_arbit_en"]
    #[inline(always)]
    pub fn arbit_en(&mut self) -> ARBIT_EN_W<'_, ARBIT_CFG_SPEC> {
        ARBIT_EN_W::new(self, 16)
    }
}
#[doc = "arbit cfg\n\nYou can [`read`](crate::Reg::read) this register and get [`arbit_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`arbit_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct ARBIT_CFG_SPEC;
impl crate::RegisterSpec for ARBIT_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`arbit_cfg::R`](R) reader structure"]
impl crate::Readable for ARBIT_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`arbit_cfg::W`](W) writer structure"]
impl crate::Writable for ARBIT_CFG_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets ARBIT_CFG to value 0"]
impl crate::Resettable for ARBIT_CFG_SPEC {}
