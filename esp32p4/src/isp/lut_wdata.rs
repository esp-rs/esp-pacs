///Register `LUT_WDATA` reader
pub type R = crate::R<LUT_WDATA_SPEC>;
///Register `LUT_WDATA` writer
pub type W = crate::W<LUT_WDATA_SPEC>;
///Field `LUT_WDATA` reader - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register
pub type LUT_WDATA_R = crate::FieldReader<u32>;
///Field `LUT_WDATA` writer - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register
pub type LUT_WDATA_W<'a, REG> = crate::FieldWriter<'a, REG, 32, u32>;
impl R {
    ///Bits 0:31 - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register
    #[inline(always)]
    pub fn lut_wdata(&self) -> LUT_WDATA_R {
        LUT_WDATA_R::new(self.bits)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("LUT_WDATA")
            .field("lut_wdata", &self.lut_wdata())
            .finish()
    }
}
impl W {
    ///Bits 0:31 - this field configures the write data of lut. please initial ISP_LUT_WDATA before write ISP_LUT_CMD register
    #[inline(always)]
    #[must_use]
    pub fn lut_wdata(&mut self) -> LUT_WDATA_W<LUT_WDATA_SPEC> {
        LUT_WDATA_W::new(self, 0)
    }
}
/**LUT write data register

You can [`read`](crate::generic::Reg::read) this register and get [`lut_wdata::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut_wdata::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUT_WDATA_SPEC;
impl crate::RegisterSpec for LUT_WDATA_SPEC {
    type Ux = u32;
}
///`read()` method returns [`lut_wdata::R`](R) reader structure
impl crate::Readable for LUT_WDATA_SPEC {}
///`write(|w| ..)` method takes [`lut_wdata::W`](W) writer structure
impl crate::Writable for LUT_WDATA_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LUT_WDATA to value 0
impl crate::Resettable for LUT_WDATA_SPEC {
    const RESET_VALUE: u32 = 0;
}
