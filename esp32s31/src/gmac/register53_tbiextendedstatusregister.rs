#[doc = "Register `REGISTER53_TBIEXTENDEDSTATUSREGISTER` reader"]
pub type R = crate::R<REGISTER53_TBIEXTENDEDSTATUSREGISTER_SPEC>;
#[doc = "Field `GHD` reader - 1000BASEX HalfDuplex Capable This bit indicates that the MAC is able to perform the halfduplex and 1000BASEX operations This bit is always low when the MAC is configured for the fullduplexonly operation during core configuration"]
pub type GHD_R = crate::BitReader;
#[doc = "Field `GFD` reader - 1000BASEX FullDuplex Capable This bit indicates that the MAC is able to perform the fullduplex and 1000BASEX operations"]
pub type GFD_R = crate::BitReader;
impl R {
    #[doc = "Bit 14 - 1000BASEX HalfDuplex Capable This bit indicates that the MAC is able to perform the halfduplex and 1000BASEX operations This bit is always low when the MAC is configured for the fullduplexonly operation during core configuration"]
    #[inline(always)]
    pub fn ghd(&self) -> GHD_R {
        GHD_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - 1000BASEX FullDuplex Capable This bit indicates that the MAC is able to perform the fullduplex and 1000BASEX operations"]
    #[inline(always)]
    pub fn gfd(&self) -> GFD_R {
        GFD_R::new(((self.bits >> 15) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("REGISTER53_TBIEXTENDEDSTATUSREGISTER")
            .field("ghd", &self.ghd())
            .field("gfd", &self.gfd())
            .finish()
    }
}
#[doc = "Indicates all modes of operation of the MAC This register is present only when you select the TBI or RTBI interface in coreConsultant\n\nYou can [`read`](crate::Reg::read) this register and get [`register53_tbiextendedstatusregister::R`](R). See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct REGISTER53_TBIEXTENDEDSTATUSREGISTER_SPEC;
impl crate::RegisterSpec for REGISTER53_TBIEXTENDEDSTATUSREGISTER_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`register53_tbiextendedstatusregister::R`](R) reader structure"]
impl crate::Readable for REGISTER53_TBIEXTENDEDSTATUSREGISTER_SPEC {}
#[doc = "`reset()` method sets REGISTER53_TBIEXTENDEDSTATUSREGISTER to value 0xc000"]
impl crate::Resettable for REGISTER53_TBIEXTENDEDSTATUSREGISTER_SPEC {
    const RESET_VALUE: u32 = 0xc000;
}
