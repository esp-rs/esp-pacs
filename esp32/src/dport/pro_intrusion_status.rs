///Register `PRO_INTRUSION_STATUS` reader
pub type R = crate::R<PRO_INTRUSION_STATUS_SPEC>;
///Field `PRO_INTRUSION_RECORD` reader -
pub type PRO_INTRUSION_RECORD_R = crate::FieldReader;
impl R {
    ///Bits 0:3
    #[inline(always)]
    pub fn pro_intrusion_record(&self) -> PRO_INTRUSION_RECORD_R {
        PRO_INTRUSION_RECORD_R::new((self.bits & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PRO_INTRUSION_STATUS")
            .field("pro_intrusion_record", &self.pro_intrusion_record())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`pro_intrusion_status::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct PRO_INTRUSION_STATUS_SPEC;
impl crate::RegisterSpec for PRO_INTRUSION_STATUS_SPEC {
    type Ux = u32;
}
///`read()` method returns [`pro_intrusion_status::R`](R) reader structure
impl crate::Readable for PRO_INTRUSION_STATUS_SPEC {}
///`reset()` method sets PRO_INTRUSION_STATUS to value 0
impl crate::Resettable for PRO_INTRUSION_STATUS_SPEC {
    const RESET_VALUE: u32 = 0;
}
