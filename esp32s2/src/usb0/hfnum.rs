///Register `HFNUM` reader
pub type R = crate::R<HFNUM_SPEC>;
///Field `FRNUM` reader -
pub type FRNUM_R = crate::FieldReader<u16>;
///Field `FRREM` reader -
pub type FRREM_R = crate::FieldReader<u16>;
impl R {
    ///Bits 0:13
    #[inline(always)]
    pub fn frnum(&self) -> FRNUM_R {
        FRNUM_R::new((self.bits & 0x3fff) as u16)
    }
    ///Bits 16:31
    #[inline(always)]
    pub fn frrem(&self) -> FRREM_R {
        FRREM_R::new(((self.bits >> 16) & 0xffff) as u16)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HFNUM")
            .field("frnum", &self.frnum())
            .field("frrem", &self.frrem())
            .finish()
    }
}
/**

You can [`read`](crate::generic::Reg::read) this register and get [`hfnum::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct HFNUM_SPEC;
impl crate::RegisterSpec for HFNUM_SPEC {
    type Ux = u32;
}
///`read()` method returns [`hfnum::R`](R) reader structure
impl crate::Readable for HFNUM_SPEC {}
///`reset()` method sets HFNUM to value 0x3fff
impl crate::Resettable for HFNUM_SPEC {
    const RESET_VALUE: u32 = 0x3fff;
}
