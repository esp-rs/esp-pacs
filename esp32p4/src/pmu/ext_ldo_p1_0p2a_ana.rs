///Register `EXT_LDO_P1_0P2A_ANA` reader
pub type R = crate::R<EXT_LDO_P1_0P2A_ANA_SPEC>;
///Register `EXT_LDO_P1_0P2A_ANA` writer
pub type W = crate::W<EXT_LDO_P1_0P2A_ANA_SPEC>;
///Field `ANA_0P2A_MUL_1` reader - need_des
pub type ANA_0P2A_MUL_1_R = crate::FieldReader;
///Field `ANA_0P2A_MUL_1` writer - need_des
pub type ANA_0P2A_MUL_1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
///Field `ANA_0P2A_EN_VDET_1` reader - need_des
pub type ANA_0P2A_EN_VDET_1_R = crate::BitReader;
///Field `ANA_0P2A_EN_VDET_1` writer - need_des
pub type ANA_0P2A_EN_VDET_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANA_0P2A_EN_CUR_LIM_1` reader - need_des
pub type ANA_0P2A_EN_CUR_LIM_1_R = crate::BitReader;
///Field `ANA_0P2A_EN_CUR_LIM_1` writer - need_des
pub type ANA_0P2A_EN_CUR_LIM_1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `ANA_0P2A_DREF_1` reader - need_des
pub type ANA_0P2A_DREF_1_R = crate::FieldReader;
///Field `ANA_0P2A_DREF_1` writer - need_des
pub type ANA_0P2A_DREF_1_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    ///Bits 23:25 - need_des
    #[inline(always)]
    pub fn ana_0p2a_mul_1(&self) -> ANA_0P2A_MUL_1_R {
        ANA_0P2A_MUL_1_R::new(((self.bits >> 23) & 7) as u8)
    }
    ///Bit 26 - need_des
    #[inline(always)]
    pub fn ana_0p2a_en_vdet_1(&self) -> ANA_0P2A_EN_VDET_1_R {
        ANA_0P2A_EN_VDET_1_R::new(((self.bits >> 26) & 1) != 0)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    pub fn ana_0p2a_en_cur_lim_1(&self) -> ANA_0P2A_EN_CUR_LIM_1_R {
        ANA_0P2A_EN_CUR_LIM_1_R::new(((self.bits >> 27) & 1) != 0)
    }
    ///Bits 28:31 - need_des
    #[inline(always)]
    pub fn ana_0p2a_dref_1(&self) -> ANA_0P2A_DREF_1_R {
        ANA_0P2A_DREF_1_R::new(((self.bits >> 28) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("EXT_LDO_P1_0P2A_ANA")
            .field("ana_0p2a_mul_1", &self.ana_0p2a_mul_1())
            .field("ana_0p2a_en_vdet_1", &self.ana_0p2a_en_vdet_1())
            .field("ana_0p2a_en_cur_lim_1", &self.ana_0p2a_en_cur_lim_1())
            .field("ana_0p2a_dref_1", &self.ana_0p2a_dref_1())
            .finish()
    }
}
impl W {
    ///Bits 23:25 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ana_0p2a_mul_1(&mut self) -> ANA_0P2A_MUL_1_W<EXT_LDO_P1_0P2A_ANA_SPEC> {
        ANA_0P2A_MUL_1_W::new(self, 23)
    }
    ///Bit 26 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ana_0p2a_en_vdet_1(&mut self) -> ANA_0P2A_EN_VDET_1_W<EXT_LDO_P1_0P2A_ANA_SPEC> {
        ANA_0P2A_EN_VDET_1_W::new(self, 26)
    }
    ///Bit 27 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ana_0p2a_en_cur_lim_1(&mut self) -> ANA_0P2A_EN_CUR_LIM_1_W<EXT_LDO_P1_0P2A_ANA_SPEC> {
        ANA_0P2A_EN_CUR_LIM_1_W::new(self, 27)
    }
    ///Bits 28:31 - need_des
    #[inline(always)]
    #[must_use]
    pub fn ana_0p2a_dref_1(&mut self) -> ANA_0P2A_DREF_1_W<EXT_LDO_P1_0P2A_ANA_SPEC> {
        ANA_0P2A_DREF_1_W::new(self, 28)
    }
}
/**need_des

You can [`read`](crate::generic::Reg::read) this register and get [`ext_ldo_p1_0p2a_ana::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`ext_ldo_p1_0p2a_ana::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct EXT_LDO_P1_0P2A_ANA_SPEC;
impl crate::RegisterSpec for EXT_LDO_P1_0P2A_ANA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`ext_ldo_p1_0p2a_ana::R`](R) reader structure
impl crate::Readable for EXT_LDO_P1_0P2A_ANA_SPEC {}
///`write(|w| ..)` method takes [`ext_ldo_p1_0p2a_ana::W`](W) writer structure
impl crate::Writable for EXT_LDO_P1_0P2A_ANA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets EXT_LDO_P1_0P2A_ANA to value 0xa000_0000
impl crate::Resettable for EXT_LDO_P1_0P2A_ANA_SPEC {
    const RESET_VALUE: u32 = 0xa000_0000;
}
