///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `DONE` reader - x
pub type DONE_R = crate::BitReader;
///Field `ERR` reader - x
pub type ERR_R = crate::BitReader;
impl R {
    ///Bit 0 - x
    #[inline(always)]
    pub fn done(&self) -> DONE_R {
        DONE_R::new((self.bits & 1) != 0)
    }
    ///Bit 1 - x
    #[inline(always)]
    pub fn err(&self) -> ERR_R {
        ERR_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("done", &self.done())
            .field("err", &self.err())
            .finish()
    }
}
/**x

You can [`read`](crate::generic::Reg::read) this register and get [`int_st::R`](R).  See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct INT_ST_SPEC;
impl crate::RegisterSpec for INT_ST_SPEC {
    type Ux = u32;
}
///`read()` method returns [`int_st::R`](R) reader structure
impl crate::Readable for INT_ST_SPEC {}
///`reset()` method sets INT_ST to value 0
impl crate::Resettable for INT_ST_SPEC {
    const RESET_VALUE: u32 = 0;
}
