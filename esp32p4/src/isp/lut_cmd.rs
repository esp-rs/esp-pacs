///Register `LUT_CMD` writer
pub type W = crate::W<LUT_CMD_SPEC>;
///Field `LUT_ADDR` writer - this field configures the lut access addr, when select lsc lut, \[11:10\]:00 sel gb_b lut, 01 sel r_gr lut
pub type LUT_ADDR_W<'a, REG> = crate::FieldWriter<'a, REG, 12, u16>;
///Field `LUT_NUM` writer - this field configures the lut selection. 0000:LSC LUT 0001:DPC LUT
pub type LUT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
///Field `LUT_CMD` writer - this bit configures the access event of lut. 0:rd 1: wr
pub type LUT_CMD_W<'a, REG> = crate::BitWriter<'a, REG>;
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<LUT_CMD_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        write!(f, "(not readable)")
    }
}
impl W {
    ///Bits 0:11 - this field configures the lut access addr, when select lsc lut, \[11:10\]:00 sel gb_b lut, 01 sel r_gr lut
    #[inline(always)]
    #[must_use]
    pub fn lut_addr(&mut self) -> LUT_ADDR_W<LUT_CMD_SPEC> {
        LUT_ADDR_W::new(self, 0)
    }
    ///Bits 12:15 - this field configures the lut selection. 0000:LSC LUT 0001:DPC LUT
    #[inline(always)]
    #[must_use]
    pub fn lut_num(&mut self) -> LUT_NUM_W<LUT_CMD_SPEC> {
        LUT_NUM_W::new(self, 12)
    }
    ///Bit 16 - this bit configures the access event of lut. 0:rd 1: wr
    #[inline(always)]
    #[must_use]
    pub fn lut_cmd(&mut self) -> LUT_CMD_W<LUT_CMD_SPEC> {
        LUT_CMD_W::new(self, 16)
    }
}
/**LUT command register

You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`lut_cmd::W`](W). See [API](https://docs.rs/svd2rust/#read--modify--write-api).*/
pub struct LUT_CMD_SPEC;
impl crate::RegisterSpec for LUT_CMD_SPEC {
    type Ux = u32;
}
///`write(|w| ..)` method takes [`lut_cmd::W`](W) writer structure
impl crate::Writable for LUT_CMD_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
///`reset()` method sets LUT_CMD to value 0
impl crate::Resettable for LUT_CMD_SPEC {
    const RESET_VALUE: u32 = 0;
}
