///Register `INT_ST` reader
pub type R = crate::R<INT_ST_SPEC>;
///Field `PAD_COMP` reader - Pad compare masked interrupt
pub type PAD_COMP_R = crate::BitReader;
impl R {
    ///Bit 0 - Pad compare masked interrupt
    #[inline(always)]
    pub fn pad_comp(&self) -> PAD_COMP_R {
        PAD_COMP_R::new((self.bits & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("INT_ST")
            .field("pad_comp", &self.pad_comp())
            .finish()
    }
}
/**GPIOSD interrupt masked register

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
