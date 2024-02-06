#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SR_EOF_INT_ST` reader - The raw interrupt status bit for the PPA_SR_EOF_INT interrupt."]
pub type SR_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `BLEND_EOF_INT_ST` reader - The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_INT_ST_R = crate::BitReader;
#[doc = "Field `SR_PARAM_CFG_ERR_INT_ST` reader - The raw interrupt status bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
pub type SR_PARAM_CFG_ERR_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn sr_eof_int_st(&self) -> SR_EOF_INT_ST_R {
        SR_EOF_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof_int_st(&self) -> BLEND_EOF_INT_ST_R {
        BLEND_EOF_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the PPA_SR_RX_YSCAL_ERR_INT interrupt."]
    #[inline(always)]
    pub fn sr_param_cfg_err_int_st(&self) -> SR_PARAM_CFG_ERR_INT_ST_R {
        SR_PARAM_CFG_ERR_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "sr_eof_int_st",
                &format_args!("{}", self.sr_eof_int_st().bit()),
            )
            .field(
                "blend_eof_int_st",
                &format_args!("{}", self.blend_eof_int_st().bit()),
            )
            .field(
                "sr_param_cfg_err_int_st",
                &format_args!("{}", self.sr_param_cfg_err_int_st().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<INT_ST_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
#[doc = "Masked interrupt\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
