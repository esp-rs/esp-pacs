///Register `INT_ENA` reader
pub type R = crate::R<INT_ENA_SPEC>;
///Register `INT_ENA` writer
pub type W = crate::W<INT_ENA_SPEC>;
///Field `IN_DONE` reader - The interrupt enable bit for the IN_DONE_CH_INT interrupt.
pub type IN_DONE_R = crate::BitReader;
///Field `IN_DONE` writer - The interrupt enable bit for the IN_DONE_CH_INT interrupt.
pub type IN_DONE_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `IN_SUC_EOF` reader - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
pub type IN_SUC_EOF_R = crate::BitReader;
///Field `IN_SUC_EOF` writer - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
pub type IN_SUC_EOF_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INFIFO_OVF_L1` reader - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
pub type INFIFO_OVF_L1_R = crate::BitReader;
///Field `INFIFO_OVF_L1` writer - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
pub type INFIFO_OVF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `INFIFO_UDF_L1` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
pub type INFIFO_UDF_L1_R = crate::BitReader;
///Field `INFIFO_UDF_L1` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
pub type INFIFO_UDF_L1_W<'a, REG> = crate::BitWriter<'a, REG>;
///Field `FETCH_MB_COL_CNT_OVF` reader - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
pub type FETCH_MB_COL_CNT_OVF_R = crate::BitReader;
///Field `FETCH_MB_COL_CNT_OVF` writer - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
pub type FETCH_MB_COL_CNT_OVF_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    ///Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    pub fn in_done(&self) -> IN_DONE_R {
        IN_DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    pub fn in_suc_eof(&self) -> IN_SUC_EOF_R {
        IN_SUC_EOF_R::new(((self.bits >> 1) & 1) != 0)
    }
    ///Bit 2 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_ovf_l1(&self) -> INFIFO_OVF_L1_R {
        INFIFO_OVF_L1_R::new(((self.bits >> 2) & 1) != 0)
    }
    ///Bit 3 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn infifo_udf_l1(&self) -> INFIFO_UDF_L1_R {
        INFIFO_UDF_L1_R::new(((self.bits >> 3) & 1) != 0)
    }
    ///Bit 4 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    pub fn fetch_mb_col_cnt_ovf(&self) -> FETCH_MB_COL_CNT_OVF_R {
        FETCH_MB_COL_CNT_OVF_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ENA")
            .field("in_done", &self.in_done())
            .field("in_suc_eof", &self.in_suc_eof())
            .field("infifo_ovf_l1", &self.infifo_ovf_l1())
            .field("infifo_udf_l1", &self.infifo_udf_l1())
            .field("fetch_mb_col_cnt_ovf", &self.fetch_mb_col_cnt_ovf())
            .finish()
    }
}
impl W {
    ///Bit 0 - The interrupt enable bit for the IN_DONE_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_done(&mut self) -> IN_DONE_W<INT_ENA_SPEC> {
        IN_DONE_W::new(self, 0)
    }
    ///Bit 1 - The interrupt enable bit for the IN_SUC_EOF_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn in_suc_eof(&mut self) -> IN_SUC_EOF_W<INT_ENA_SPEC> {
        IN_SUC_EOF_W::new(self, 1)
    }
    ///Bit 2 - The interrupt enable bit for the INFIFO_OVF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_ovf_l1(&mut self) -> INFIFO_OVF_L1_W<INT_ENA_SPEC> {
        INFIFO_OVF_L1_W::new(self, 2)
    }
    ///Bit 3 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn infifo_udf_l1(&mut self) -> INFIFO_UDF_L1_W<INT_ENA_SPEC> {
        INFIFO_UDF_L1_W::new(self, 3)
    }
    ///Bit 4 - The interrupt enable bit for the INFIFO_UDF_L1_CH_INT interrupt.
    #[inline(always)]
    #[must_use]
    pub fn fetch_mb_col_cnt_ovf(&mut self) -> FETCH_MB_COL_CNT_OVF_W<INT_ENA_SPEC> {
        FETCH_MB_COL_CNT_OVF_W::new(self, 4)
    }
}
/**RX CH5 interrupt ena register

You can [`read`](crate::generic::Reg::read) this register and get [`int_ena::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`int_ena::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ENA_SPEC;
impl crate::RegisterSpec for INT_ENA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_ena::R`](R) reader structure
impl crate::Readable for INT_ENA_SPEC {}
///`write(|w| ..)` method takes [`int_ena::W`](W) writer structure
impl crate::Writable for INT_ENA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets INT_ENA to value 0
impl crate::Resettable for INT_ENA_SPEC {
    const RESET_VALUE: u32 = 0;
}
