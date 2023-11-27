#[doc = "Register `INT_ST` reader"]
pub type R = crate::R<INT_ST_SPEC>;
#[doc = "Field `VADR_NUM_GT_INT_ST` reader - reg_vadr_num is greater than real interrupt st."]
pub type VADR_NUM_GT_INT_ST_R = crate::BitReader;
#[doc = "Field `VADR_NUM_LT_INT_ST` reader - reg_vadr_num is less than real interrupt st."]
pub type VADR_NUM_LT_INT_ST_R = crate::BitReader;
#[doc = "Field `DISCARD_INT_ST` reader - an incomplete frame of data was sent interrupt st."]
pub type DISCARD_INT_ST_R = crate::BitReader;
#[doc = "Field `CSI_BUF_OVERRUN_INT_ST` reader - buffer overrun interrupt st."]
pub type CSI_BUF_OVERRUN_INT_ST_R = crate::BitReader;
#[doc = "Field `CSI_ASYNC_FIFO_OVF_INT_ST` reader - buffer overflow interrupt st."]
pub type CSI_ASYNC_FIFO_OVF_INT_ST_R = crate::BitReader;
#[doc = "Field `DMA_CFG_HAS_UPDATED_INT_ST` reader - dma configuration update complete interrupt st."]
pub type DMA_CFG_HAS_UPDATED_INT_ST_R = crate::BitReader;
impl R {
    #[doc = "Bit 0 - reg_vadr_num is greater than real interrupt st."]
    #[inline(always)]
    pub fn vadr_num_gt_int_st(&self) -> VADR_NUM_GT_INT_ST_R {
        VADR_NUM_GT_INT_ST_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - reg_vadr_num is less than real interrupt st."]
    #[inline(always)]
    pub fn vadr_num_lt_int_st(&self) -> VADR_NUM_LT_INT_ST_R {
        VADR_NUM_LT_INT_ST_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - an incomplete frame of data was sent interrupt st."]
    #[inline(always)]
    pub fn discard_int_st(&self) -> DISCARD_INT_ST_R {
        DISCARD_INT_ST_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - buffer overrun interrupt st."]
    #[inline(always)]
    pub fn csi_buf_overrun_int_st(&self) -> CSI_BUF_OVERRUN_INT_ST_R {
        CSI_BUF_OVERRUN_INT_ST_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - buffer overflow interrupt st."]
    #[inline(always)]
    pub fn csi_async_fifo_ovf_int_st(&self) -> CSI_ASYNC_FIFO_OVF_INT_ST_R {
        CSI_ASYNC_FIFO_OVF_INT_ST_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - dma configuration update complete interrupt st."]
    #[inline(always)]
    pub fn dma_cfg_has_updated_int_st(&self) -> DMA_CFG_HAS_UPDATED_INT_ST_R {
        DMA_CFG_HAS_UPDATED_INT_ST_R::new(((self.bits >> 5) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field(
                "vadr_num_gt_int_st",
                &format_args!("{}", self.vadr_num_gt_int_st().bit()),
            )
            .field(
                "vadr_num_lt_int_st",
                &format_args!("{}", self.vadr_num_lt_int_st().bit()),
            )
            .field(
                "discard_int_st",
                &format_args!("{}", self.discard_int_st().bit()),
            )
            .field(
                "csi_buf_overrun_int_st",
                &format_args!("{}", self.csi_buf_overrun_int_st().bit()),
            )
            .field(
                "csi_async_fifo_ovf_int_st",
                &format_args!("{}", self.csi_async_fifo_ovf_int_st().bit()),
            )
            .field(
                "dma_cfg_has_updated_int_st",
                &format_args!("{}", self.dma_cfg_has_updated_int_st().bit()),
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
#[doc = "csi bridge interrupt st.\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`int_st::R`](R) reader structure"]
impl crate::Readable for INT_ST_SPEC {}
#[doc = "`reset()` method sets INT_ST to value 0"]
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
