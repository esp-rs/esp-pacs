#[doc = "Register `CH%s_GAMMA_RD_DATA` reader"]
pub struct R(crate::R<CH_GAMMA_RD_DATA_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<CH_GAMMA_RD_DATA_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<CH_GAMMA_RD_DATA_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<CH_GAMMA_RD_DATA_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `CH_GAMMA_RD_DATA` reader - Ledc ch%s gamma ram read data."]
pub type CH_GAMMA_RD_DATA_R = crate::FieldReader<u32>;
impl R {
    #[doc = "Bits 0:30 - Ledc ch%s gamma ram read data."]
    #[inline(always)]
    pub fn ch_gamma_rd_data(&self) -> CH_GAMMA_RD_DATA_R {
        CH_GAMMA_RD_DATA_R::new(self.bits & 0x7fff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CH_GAMMA_RD_DATA")
            .field(
                "ch_gamma_rd_data",
                &format_args!("{}", self.ch_gamma_rd_data().bits()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CH_GAMMA_RD_DATA_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "Ledc ch%s gamma ram read data register.\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [ch_gamma_rd_data](index.html) module"]
pub struct CH_GAMMA_RD_DATA_SPEC;
impl crate::RegisterSpec for CH_GAMMA_RD_DATA_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [ch_gamma_rd_data::R](R) reader structure"]
impl crate::Readable for CH_GAMMA_RD_DATA_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets CH%s_GAMMA_RD_DATA to value 0"]
impl crate::Resettable for CH_GAMMA_RD_DATA_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
