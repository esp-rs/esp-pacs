#[doc = "Register `EXT_LDO_P0_0P3A_ANA` reader"]
pub type R = crate::R<EXT_LDO_P0_0P3A_ANA_SPEC>;
#[doc = "Register `EXT_LDO_P0_0P3A_ANA` writer"]
pub type W = crate::W<EXT_LDO_P0_0P3A_ANA_SPEC>;
#[doc = "Field `ANA_0P3A_MUL_0` reader - need_des"]
pub type ANA_0P3A_MUL_0_R = crate::FieldReader;
#[doc = "Field `ANA_0P3A_MUL_0` writer - need_des"]
pub type ANA_0P3A_MUL_0_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `ANA_0P3A_EN_VDET_0` reader - need_des"]
pub type ANA_0P3A_EN_VDET_0_R = crate::BitReader;
#[doc = "Field `ANA_0P3A_EN_VDET_0` writer - need_des"]
pub type ANA_0P3A_EN_VDET_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_0P3A_EN_CUR_LIM_0` reader - need_des"]
pub type ANA_0P3A_EN_CUR_LIM_0_R = crate::BitReader;
#[doc = "Field `ANA_0P3A_EN_CUR_LIM_0` writer - need_des"]
pub type ANA_0P3A_EN_CUR_LIM_0_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `ANA_0P3A_DREF_0` reader - need_des"]
pub type ANA_0P3A_DREF_0_R = crate::FieldReader;
#[doc = "Field `ANA_0P3A_DREF_0` writer - need_des"]
pub type ANA_0P3A_DREF_0_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_mul_0(&self) -> ANA_0P3A_MUL_0_R {
        ANA_0P3A_MUL_0_R::new(((self.bits >> 23) & 7) as u8)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_vdet_0(&self) -> ANA_0P3A_EN_VDET_0_R {
        ANA_0P3A_EN_VDET_0_R::new(((self.bits >> 26) & 1) != 0)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_cur_lim_0(&self) -> ANA_0P3A_EN_CUR_LIM_0_R {
        ANA_0P3A_EN_CUR_LIM_0_R::new(((self.bits >> 27) & 1) != 0)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_dref_0(&self) -> ANA_0P3A_DREF_0_R {
        ANA_0P3A_DREF_0_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_P0_0P3A_ANA")
            .field("ana_0p3a_mul_0", &self.ana_0p3a_mul_0())
            .field("ana_0p3a_en_vdet_0", &self.ana_0p3a_en_vdet_0())
            .field("ana_0p3a_en_cur_lim_0", &self.ana_0p3a_en_cur_lim_0())
            .field("ana_0p3a_dref_0", &self.ana_0p3a_dref_0())
            .finish()
    }
}
impl W {
    #[doc = "Bits 23:25 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_mul_0(&mut self) -> ANA_0P3A_MUL_0_W<EXT_LDO_P0_0P3A_ANA_SPEC> {
        ANA_0P3A_MUL_0_W::new(self, 23)
    }
    #[doc = "Bit 26 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_vdet_0(&mut self) -> ANA_0P3A_EN_VDET_0_W<EXT_LDO_P0_0P3A_ANA_SPEC> {
        ANA_0P3A_EN_VDET_0_W::new(self, 26)
    }
    #[doc = "Bit 27 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_en_cur_lim_0(&mut self) -> ANA_0P3A_EN_CUR_LIM_0_W<EXT_LDO_P0_0P3A_ANA_SPEC> {
        ANA_0P3A_EN_CUR_LIM_0_W::new(self, 27)
    }
    #[doc = "Bits 28:31 - need_des"]
    #[inline(always)]
    pub fn ana_0p3a_dref_0(&mut self) -> ANA_0P3A_DREF_0_W<EXT_LDO_P0_0P3A_ANA_SPEC> {
        ANA_0P3A_DREF_0_W::new(self, 28)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`ext_ldo_p0_0p3a_ana::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`ext_ldo_p0_0p3a_ana::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct EXT_LDO_P0_0P3A_ANA_SPEC;
impl crate::RegisterSpec for EXT_LDO_P0_0P3A_ANA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`ext_ldo_p0_0p3a_ana::R`](R) reader structure"]
impl crate::Readable for EXT_LDO_P0_0P3A_ANA_SPEC {}
#[doc = "`write(|w| ..)` method takes [`ext_ldo_p0_0p3a_ana::W`](W) writer structure"]
impl crate::Writable for EXT_LDO_P0_0P3A_ANA_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets EXT_LDO_P0_0P3A_ANA to value 0xa000_0000"]
impl crate::Resettable for EXT_LDO_P0_0P3A_ANA_SPEC {
    const RESET_VALUE: u32 = 0xa000_0000;
}
