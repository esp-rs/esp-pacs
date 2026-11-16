#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `SRM_EOF` reader - The raw interrupt status bit for the PPA_SR_EOF_INT interrupt."]
pub type SRM_EOF_R = crate::BitReader;
#[doc = "Field `BLEND_EOF` reader - The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt."]
pub type BLEND_EOF_R = crate::BitReader;
#[doc = "Field `SRM_PARAM_CFG_ERR` reader - The raw interrupt status bit for the PPA_SR_PARAM_CFG_ERR_INT interrupt."]
pub type SRM_PARAM_CFG_ERR_R = crate::BitReader;
#[doc = "Field `BLEND_PARAM_CFG_ERR` reader - The raw interrupt status bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
pub type BLEND_PARAM_CFG_ERR_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - The raw interrupt status bit for the PPA_SR_EOF_INT interrupt."]
    #[inline(always)]
    pub fn srm_eof(&self) -> SRM_EOF_R {
        SRM_EOF_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - The raw interrupt status bit for the PPA_BLEND_EOF_INT interrupt."]
    #[inline(always)]
    pub fn blend_eof(&self) -> BLEND_EOF_R {
        BLEND_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - The raw interrupt status bit for the PPA_SR_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn srm_param_cfg_err(&self) -> SRM_PARAM_CFG_ERR_R {
        SRM_PARAM_CFG_ERR_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - The raw interrupt status bit for the PPA_BLEND_PARAM_CFG_ERR_INT interrupt."]
    #[inline(always)]
    pub fn blend_param_cfg_err(&self) -> BLEND_PARAM_CFG_ERR_R {
        BLEND_PARAM_CFG_ERR_R::new(((self.bits >> 3) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("srm_eof", &self.srm_eof())
            .field("blend_eof", &self.blend_eof())
            .field("srm_param_cfg_err", &self.srm_param_cfg_err())
            .field("blend_param_cfg_err", &self.blend_param_cfg_err())
            .finish()
    }
}
#[doc = "Masked interrupt\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
