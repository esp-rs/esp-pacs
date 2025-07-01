#[doc = "Register `PRESENT_STATE1` reader"]
pub type R = crate::R<PRESENT_STATE1_SPEC>;
#[doc = "Field `DATA_BYTE_CNT` reader - Present transfer data byte cnt: tx data byte cnt if write rx data byte cnt if read ibi data byte cnt if IBI handle."]
pub type DATA_BYTE_CNT_R = crate::FieldReader<u16>;
impl R {
    #[doc = "Bits 0:15 - Present transfer data byte cnt: tx data byte cnt if write rx data byte cnt if read ibi data byte cnt if IBI handle."]
    #[inline(always)]
    pub fn data_byte_cnt(&self) -> DATA_BYTE_CNT_R {
        DATA_BYTE_CNT_R::new((self.bits & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRESENT_STATE1")
            .field("data_byte_cnt", &self.data_byte_cnt())
            .finish()
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`present_state1::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PRESENT_STATE1_SPEC;
impl crate::RegisterSpec for PRESENT_STATE1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`present_state1::R`](R) reader structure"]
impl crate::Readable for PRESENT_STATE1_SPEC {}
#[doc = "`reset()` method sets PRESENT_STATE1 to value 0"]
impl crate::Resettable for PRESENT_STATE1_SPEC {}
