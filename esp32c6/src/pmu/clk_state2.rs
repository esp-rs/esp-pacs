#[doc = "Register `CLK_STATE2` reader"]
pub struct R(crate::R<CLK_STATE2_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CLK_STATE2_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CLK_STATE2_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CLK_STATE2_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `ICG_APB_EN_STATE` reader - need_des"]
pub type ICG_APB_EN_STATE_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:31 - need_des"]
    #[inline(always)]
    pub fn icg_apb_en_state(&self) -> ICG_APB_EN_STATE_R {
        ICG_APB_EN_STATE_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK_STATE2")
            .field(
                "icg_apb_en_state",
                &format_args!("{}", self.icg_apb_en_state().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLK_STATE2_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "need_des\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [clk_state2](index.html) module"]
pub struct CLK_STATE2_SPEC;
impl crate::RegisterSpec for CLK_STATE2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [clk_state2::R](R) reader structure"]
impl crate::Readable for CLK_STATE2_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CLK_STATE2 to value 0xffff_ffff"]
impl crate::Resettable for CLK_STATE2_SPEC {
    const RESET_VALUE: Self::Ux = 0xffff_ffff;
}
