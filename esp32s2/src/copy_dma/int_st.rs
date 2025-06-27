#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `IN_DONE` reader - This is the masked interrupt bit for IN_DONE interrupt when IN_DONE is enabled."]
pub type IN_DONE_R = crate::BitReader;
#[doc = "Field `IN_SUC_EOF` reader - This is the masked interrupt bit for IN_SUC_EOF interrupt when IN_SUC_EOF is enabled."]
pub type IN_SUC_EOF_R = crate::BitReader;
#[doc = "Field `OUT_DONE` reader - This is the masked interrupt bit for OUT_DONE interrupt when OUT_DONE is enabled."]
pub type OUT_DONE_R = crate::BitReader;
#[doc = "Field `OUT_EOF` reader - This is the masked interrupt bit for OUT_EOF interrupt when OUT_EOF is enabled."]
pub type OUT_EOF_R = crate::BitReader;
#[doc = "Field `IN_DSCR_ERR` reader - This is the masked interrupt bit for IN_DSCR_ERR interrupt when IN_DSCR_ERR is enabled."]
pub type IN_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `OUT_DSCR_ERR` reader - This is the masked interrupt bit for OUT_DSCR_ERR interrupt when OUT_DSCR_ERR is enabled."]
pub type OUT_DSCR_ERR_R = crate::BitReader;
#[doc = "Field `IN_DSCR_EMPTY` reader - This is the masked interrupt bit for IN_DSCR_EMPTY interrupt when IN_DSCR_EMPTY is enabled."]
pub type IN_DSCR_EMPTY_R = crate::BitReader;
#[doc = "Field `OUT_TOTAL_EOF` reader - This is the masked interrupt bit for OUT_TOTAL_EOF interrupt when OUT_TOTAL_EOF is enabled."]
pub type OUT_TOTAL_EOF_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - This is the masked interrupt bit for IN_DONE interrupt when IN_DONE is enabled."]
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This is the masked interrupt bit for IN_SUC_EOF interrupt when IN_SUC_EOF is enabled."]
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - This is the masked interrupt bit for OUT_DONE interrupt when OUT_DONE is enabled."]
    #[inline(always)]
    pub fn out_done(&self) -> OUT_DONE_R {
        OUT_DONE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - This is the masked interrupt bit for OUT_EOF interrupt when OUT_EOF is enabled."]
    #[inline(always)]
    pub fn out_eof(&self) -> OUT_EOF_R {
        OUT_EOF_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - This is the masked interrupt bit for IN_DSCR_ERR interrupt when IN_DSCR_ERR is enabled."]
    #[inline(always)]
    pub fn in_dscr_err(&self) -> IN_DSCR_ERR_R {
        IN_DSCR_ERR_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - This is the masked interrupt bit for OUT_DSCR_ERR interrupt when OUT_DSCR_ERR is enabled."]
    #[inline(always)]
    pub fn out_dscr_err(&self) -> OUT_DSCR_ERR_R {
        OUT_DSCR_ERR_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - This is the masked interrupt bit for IN_DSCR_EMPTY interrupt when IN_DSCR_EMPTY is enabled."]
    #[inline(always)]
    pub fn in_dscr_empty(&self) -> IN_DSCR_EMPTY_R {
        IN_DSCR_EMPTY_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - This is the masked interrupt bit for OUT_TOTAL_EOF interrupt when OUT_TOTAL_EOF is enabled."]
    #[inline(always)]
    pub fn out_total_eof(&self) -> OUT_TOTAL_EOF_R {
        OUT_TOTAL_EOF_R::new(((self.bits >> 7) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("out_done", &self.out_done())
            .field("out_eof", &self.out_eof())
            .field("in_dscr_err", &self.in_dscr_err())
            .field("out_dscr_err", &self.out_dscr_err())
            .field("in_dscr_empty", &self.in_dscr_empty())
            .field("out_total_eof", &self.out_total_eof())
            .finish()
    }
}
#[doc = "Masked interrupt status\n\nYou can [`read`](crate::Reg::read) this register and get [`int_st::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {}
