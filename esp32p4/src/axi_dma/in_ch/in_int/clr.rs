#[doc = "Register `CLR` writer"]
pub type W = crate::W<CLR_SPEC>;
#[doc = "Field `IN_DONE` writer - Set this bit to clear the IN_DONE_CH_INT interrupt."]
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_SUC_EOF` writer - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_ERR_EOF` writer - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
pub type IN_ERR_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_ERR` writer - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
pub type IN_DSCR_ERR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `IN_DSCR_EMPTY` writer - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
pub type IN_DSCR_EMPTY_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_OVF` writer - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
pub type INFIFO_L1_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L1_UDF` writer - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
pub type INFIFO_L1_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_OVF` writer - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
pub type INFIFO_L2_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L2_UDF` writer - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
pub type INFIFO_L2_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_OVF` writer - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt."]
pub type INFIFO_L3_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `INFIFO_L3_UDF` writer - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt."]
pub type INFIFO_L3_UDF_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<CLR_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    #[doc = "Bit 0 - Set this bit to clear the IN_DONE_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<CLR_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Set this bit to clear the IN_SUC_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<CLR_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    #[doc = "Bit 2 - Set this bit to clear the IN_ERR_EOF_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_err_eof(&mut self) -> IN_ERR_EOF_W<CLR_SPEC> {
        IN_ERR_EOF_W::new(self, 2)
    }
    #[doc = "Bit 3 - Set this bit to clear the IN_DSCR_ERR_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_err(&mut self) -> IN_DSCR_ERR_W<CLR_SPEC> {
        IN_DSCR_ERR_W::new(self, 3)
    }
    #[doc = "Bit 4 - Set this bit to clear the IN_DSCR_EMPTY_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn in_dscr_empty(&mut self) -> IN_DSCR_EMPTY_W<CLR_SPEC> {
        IN_DSCR_EMPTY_W::new(self, 4)
    }
    #[doc = "Bit 5 - Set this bit to clear the INFIFO_OVF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l1_ovf(&mut self) -> INFIFO_L1_OVF_W<CLR_SPEC> {
        INFIFO_L1_OVF_W::new(self, 5)
    }
    #[doc = "Bit 6 - Set this bit to clear the INFIFO_UDF_L1_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l1_udf(&mut self) -> INFIFO_L1_UDF_W<CLR_SPEC> {
        INFIFO_L1_UDF_W::new(self, 6)
    }
    #[doc = "Bit 7 - Set this bit to clear the INFIFO_OVF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l2_ovf(&mut self) -> INFIFO_L2_OVF_W<CLR_SPEC> {
        INFIFO_L2_OVF_W::new(self, 7)
    }
    #[doc = "Bit 8 - Set this bit to clear the INFIFO_UDF_L2_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l2_udf(&mut self) -> INFIFO_L2_UDF_W<CLR_SPEC> {
        INFIFO_L2_UDF_W::new(self, 8)
    }
    #[doc = "Bit 9 - Set this bit to clear the INFIFO_OVF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l3_ovf(&mut self) -> INFIFO_L3_OVF_W<CLR_SPEC> {
        INFIFO_L3_OVF_W::new(self, 9)
    }
    #[doc = "Bit 10 - Set this bit to clear the INFIFO_UDF_L3_CH_INT interrupt."]
    #[inline(always)]
    #[must_use]
    pub fn infifo_l3_udf(&mut self) -> INFIFO_L3_UDF_W<CLR_SPEC> {
        INFIFO_L3_UDF_W::new(self, 10)
    }
}
#[doc = "Interrupt clear bits of channel 0\n\nYou can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clr::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLR_SPEC;
impl crate::RegisterSpec for CLR_SPEC {
    type Ux = u32;
}
#[doc = "`write(|w| ..)` method takes [`clr::W`](W) writer structure"]
impl crate::Writable for CLR_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLR to value 0"]
impl crate::Resettable for CLR_SPEC {
    const RESET_VALUE: u32 = 0;
}
