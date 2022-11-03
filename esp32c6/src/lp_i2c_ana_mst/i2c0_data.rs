#[doc = "Register `I2C0_DATA` reader"]
pub struct R(crate::R<I2C0_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<I2C0_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<I2C0_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<I2C0_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Register `I2C0_DATA` writer"]
pub struct W(crate::W<I2C0_DATA_SPEC>);
impl core::ops::Deref for W {
    type Target = crate::W<I2C0_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl core::ops::DerefMut for W {
    #[inline(always)]
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.0
    }
}
impl From<crate::W<I2C0_DATA_SPEC>> for W {
    #[inline(always)]
    fn from(writer: crate::W<I2C0_DATA_SPEC>) -> Self {
        W(writer)
    }
}
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_RDATA` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_RDATA_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CLK_SEL` reader - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CLK_SEL_R = crate::FieldReader<u8, u8>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C0_CLK_SEL` writer - need_des"]
pub type LP_I2C_ANA_MAST_I2C0_CLK_SEL_W<'a, const O: u8> =
    crate::FieldWriter<'a, u32, I2C0_DATA_SPEC, u8, u8, 3, O>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_SEL` reader - need des"]
pub type LP_I2C_ANA_MAST_I2C_MST_SEL_R = crate::BitReader<bool>;
#[doc = "Field `LP_I2C_ANA_MAST_I2C_MST_SEL` writer - need des"]
pub type LP_I2C_ANA_MAST_I2C_MST_SEL_W<'a, const O: u8> =
    crate::BitWriter<'a, u32, I2C0_DATA_SPEC, bool, O>;
impl R {
    #[doc = "Bits 0:7 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_rdata(&self) -> LP_I2C_ANA_MAST_I2C0_RDATA_R {
        LP_I2C_ANA_MAST_I2C0_RDATA_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c0_clk_sel(&self) -> LP_I2C_ANA_MAST_I2C0_CLK_SEL_R {
        LP_I2C_ANA_MAST_I2C0_CLK_SEL_R::new(((self.bits >> 8) & 7) as u8)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    pub fn lp_i2c_ana_mast_i2c_mst_sel(&self) -> LP_I2C_ANA_MAST_I2C_MST_SEL_R {
        LP_I2C_ANA_MAST_I2C_MST_SEL_R::new(((self.bits >> 11) & 1) != 0)
    }
}
impl W {
    #[doc = "Bits 8:10 - need_des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c0_clk_sel(&mut self) -> LP_I2C_ANA_MAST_I2C0_CLK_SEL_W<8> {
        LP_I2C_ANA_MAST_I2C0_CLK_SEL_W::new(self)
    }
    #[doc = "Bit 11 - need des"]
    #[inline(always)]
    #[must_use]
    pub fn lp_i2c_ana_mast_i2c_mst_sel(&mut self) -> LP_I2C_ANA_MAST_I2C_MST_SEL_W<11> {
        LP_I2C_ANA_MAST_I2C_MST_SEL_W::new(self)
    }
    #[doc = "Writes raw bits to the register."]
    #[inline(always)]
    pub unsafe fn bits(&mut self, bits: u32) -> &mut Self {
        self.0.bits(bits);
        self
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read), [`write_with_zero`](crate::generic::Reg::write_with_zero), [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`modify`](crate::generic::Reg::modify). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [i2c0_data](index.html) module"]
pub struct I2C0_DATA_SPEC;
impl crate::RegisterSpec for I2C0_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [i2c0_data::R](R) reader structure"]
impl crate::Readable for I2C0_DATA_SPEC {
    type Reader = R;
}
#[doc = "`write(|w| ..)` method takes [i2c0_data::W](W) writer structure"]
impl crate::Writable for I2C0_DATA_SPEC {
    type Writer = W;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: Self::Ux = 0;
}
#[doc = "`reset()` method sets I2C0_DATA to value 0x0900"]
impl crate::Resettable for I2C0_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0x0900;
}
