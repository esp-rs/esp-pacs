#[doc = "Register `CLK_STATE1` reader"]
pub struct R(crate::R<CLK_STATE1_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_STATE1_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_STATE1_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_STATE1_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICG_FUNC_EN_STATE` reader - need_des"]
pub type ICG_FUNC_EN_STATE_R = crate::FieldReader<u32, u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn icg_func_en_state(&self) -> ICG_FUNC_EN_STATE_R {
        ICG_FUNC_EN_STATE_R::new(self.bits)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_state1](index.html) module"]
pub struct CLK_STATE1_SPEC;
impl crate::RegisterSpec for CLK_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_state1::R](R) reader structure"]
impl crate::Readable for CLK_STATE1_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_STATE1 to value 0xffff_ffff"]
impl crate::Resettable for CLK_STATE1_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
