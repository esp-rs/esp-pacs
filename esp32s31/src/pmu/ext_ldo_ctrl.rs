#[doc = "Register `EXT_LDO_CTRL` reader"]
pub type R = crate::R<EXT_LDO_CTRL_SPEC>;
#[doc = "Register `EXT_LDO_CTRL` writer"]
pub type W = crate::W<EXT_LDO_CTRL_SPEC>;
#[doc = "Field `EXT_LDO_TIE_HIGH` reader - need_des"]
pub type EXT_LDO_TIE_HIGH_R = crate::BitReader;
#[doc = "Field `EXT_LDO_TIE_HIGH` writer - need_des"]
pub type EXT_LDO_TIE_HIGH_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_LDO_EN_VDET` reader - need_des"]
pub type EXT_LDO_EN_VDET_R = crate::BitReader;
#[doc = "Field `EXT_LDO_EN_VDET` writer - need_des"]
pub type EXT_LDO_EN_VDET_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_CUR_LIM` reader - need_des"]
pub type EXT_CUR_LIM_R = crate::BitReader;
#[doc = "Field `EXT_CUR_LIM` writer - need_des"]
pub type EXT_CUR_LIM_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `EXT_LDO_MUL` reader - need_des"]
pub type EXT_LDO_MUL_R = crate::FieldReader;
#[doc = "Field `EXT_LDO_MUL` writer - need_des"]
pub type EXT_LDO_MUL_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `EXT_LDO_DREF` reader - need_des"]
pub type EXT_LDO_DREF_R = crate::FieldReader;
#[doc = "Field `EXT_LDO_DREF` writer - need_des"]
pub type EXT_LDO_DREF_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_tie_high(&self) -> EXT_LDO_TIE_HIGH_R {
        EXT_LDO_TIE_HIGH_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_en_vdet(&self) -> EXT_LDO_EN_VDET_R {
        EXT_LDO_EN_VDET_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ext_cur_lim(&self) -> EXT_CUR_LIM_R {
        EXT_CUR_LIM_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bits 3:5 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_mul(&self) -> EXT_LDO_MUL_R {
        EXT_LDO_MUL_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:9 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_dref(&self) -> EXT_LDO_DREF_R {
        EXT_LDO_DREF_R::new(((self.bits >> 6) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_CTRL")
            .field("ext_ldo_tie_high", &self.ext_ldo_tie_high())
            .field("ext_ldo_en_vdet", &self.ext_ldo_en_vdet())
            .field("ext_cur_lim", &self.ext_cur_lim())
            .field("ext_ldo_mul", &self.ext_ldo_mul())
            .field("ext_ldo_dref", &self.ext_ldo_dref())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_tie_high(&mut self) -> EXT_LDO_TIE_HIGH_W<'_, EXT_LDO_CTRL_SPEC> {
        EXT_LDO_TIE_HIGH_W::new(self, 0)
    }
    #[doc = "Bit 1 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_en_vdet(&mut self) -> EXT_LDO_EN_VDET_W<'_, EXT_LDO_CTRL_SPEC> {
        EXT_LDO_EN_VDET_W::new(self, 1)
    }
    #[doc = "Bit 2 - need_des"]
    #[inline(always)]
    pub fn ext_cur_lim(&mut self) -> EXT_CUR_LIM_W<'_, EXT_LDO_CTRL_SPEC> {
        EXT_CUR_LIM_W::new(self, 2)
    }
    #[doc = "Bits 3:5 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_mul(&mut self) -> EXT_LDO_MUL_W<'_, EXT_LDO_CTRL_SPEC> {
        EXT_LDO_MUL_W::new(self, 3)
    }
    #[doc = "Bits 6:9 - need_des"]
    #[inline(always)]
    pub fn ext_ldo_dref(&mut self) -> EXT_LDO_DREF_W<'_, EXT_LDO_CTRL_SPEC> {
        EXT_LDO_DREF_W::new(self, 6)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_ctrl::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_ctrl::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_LDO_CTRL_SPEC;
impl crate::RegisterSpec for EXT_LDO_CTRL_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_ctrl::R`](R) reader structure"]
impl crate::Readable for EXT_LDO_CTRL_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_ctrl::W`](W) writer structure"]
impl crate::Writable for EXT_LDO_CTRL_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_LDO_CTRL to value 0"]
impl crate::Resettable for EXT_LDO_CTRL_SPEC {}
