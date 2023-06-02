#[doc = "Register `MEM_CNT_STATUS` reader"]
pub struct R(crate::R<MEM_CNT_STATUS_SPEC>);
impl core::ops::Deref for R {
    type Target = crate::R<MEM_CNT_STATUS_SPEC>;
    #[inline(always)]
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
impl From<crate::R<MEM_CNT_STATUS_SPEC>> for R {
    #[inline(always)]
    fn from(reader: crate::R<MEM_CNT_STATUS_SPEC>) -> Self {
        R(reader)
    }
}
#[doc = "Field `RX_MEM_CNT` reader - refer to the rxfifo_cnt's describtion."]
pub type RX_MEM_CNT_R = crate::FieldReader;
#[doc = "Field `TX_MEM_CNT` reader - refer to the txfifo_cnt's describtion."]
pub type TX_MEM_CNT_R = crate::FieldReader;
impl R {
    #[doc = "Bits 0:2 - refer to the rxfifo_cnt's describtion."]
    #[inline(always)]
    pub fn rx_mem_cnt(&self) -> RX_MEM_CNT_R {
        RX_MEM_CNT_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - refer to the txfifo_cnt's describtion."]
    #[inline(always)]
    pub fn tx_mem_cnt(&self) -> TX_MEM_CNT_R {
        TX_MEM_CNT_R::new(((self.bits >> 3) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MEM_CNT_STATUS")
            .field("rx_mem_cnt", &format_args!("{}", self.rx_mem_cnt().bits()))
            .field("tx_mem_cnt", &format_args!("{}", self.tx_mem_cnt().bits()))
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<MEM_CNT_STATUS_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        self.read().fmt(f)
    }
}
#[doc = "\n\nThis register you can [`read`](crate::generic::Reg::read). See [API](https://docs.rs/svd2rust/#read--modify--write-api).\n\nFor information about available fields see [mem_cnt_status](index.html) module"]
pub struct MEM_CNT_STATUS_SPEC;
impl crate::RegisterSpec for MEM_CNT_STATUS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [mem_cnt_status::R](R) reader structure"]
impl crate::Readable for MEM_CNT_STATUS_SPEC {
    type Reader = R;
}
#[doc = "`reset()` method sets MEM_CNT_STATUS to value 0"]
impl crate::Resettable for MEM_CNT_STATUS_SPEC {
    const RESET_VALUE: Self::Ux = 0;
}
