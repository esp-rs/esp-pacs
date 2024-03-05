#[doc = "Register `RAW_NUM_CFG` reader"]
pub type R = crate::R<RAW_NUM_CFG_SPEC>;
#[doc = "Register `RAW_NUM_CFG` writer"]
pub type W = crate::W<RAW_NUM_CFG_SPEC>;
#[doc = "Field `RAW_NUM_TOTAL` reader - this field configures number of total pix bits/64"]
pub type RAW_NUM_TOTAL_R = crate::FieldReader<u32>;
#[doc = "Field `RAW_NUM_TOTAL` writer - this field configures number of total pix bits/64"]
pub type RAW_NUM_TOTAL_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `UNALIGN_64BIT_EN` reader - this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit"]
pub type UNALIGN_64BIT_EN_R = crate::BitReader;
#[doc = "Field `UNALIGN_64BIT_EN` writer - this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit"]
pub type UNALIGN_64BIT_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RAW_NUM_TOTAL_SET` writer - this bit configures enable of reload reg_raw_num_total to internal cnt. 0: disable, 1: enable. valid only when dsi_bridge as flow controller"]
pub type RAW_NUM_TOTAL_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - this field configures number of total pix bits/64"]
    #[inline(always)]
    pub fn raw_num_total(&self) -> RAW_NUM_TOTAL_R {
        RAW_NUM_TOTAL_R::new(self.bits & 0x003f_ffff)
    }
    #[doc = "Bit 22 - this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit"]
    #[inline(always)]
    pub fn unalign_64bit_en(&self) -> UNALIGN_64BIT_EN_R {
        UNALIGN_64BIT_EN_R::new(((self.bits >> 22) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RAW_NUM_CFG")
            .field(
                "raw_num_total",
                &format_args!("{}", self.raw_num_total().bits()),
            )
            .field(
                "unalign_64bit_en",
                &format_args!("{}", self.unalign_64bit_en().bit()),
            )
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<RAW_NUM_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - this field configures number of total pix bits/64"]
    #[inline(always)]
    #[must_use]
    pub fn raw_num_total(&mut self) -> RAW_NUM_TOTAL_W<RAW_NUM_CFG_SPEC> {
        RAW_NUM_TOTAL_W::new(self, 0)
    }
    #[doc = "Bit 22 - this field configures whether the total pix bits is a multiple of 64bits. 0: align to 64-bit, 1: unalign to 64-bit"]
    #[inline(always)]
    #[must_use]
    pub fn unalign_64bit_en(&mut self) -> UNALIGN_64BIT_EN_W<RAW_NUM_CFG_SPEC> {
        UNALIGN_64BIT_EN_W::new(self, 22)
    }
    #[doc = "Bit 31 - this bit configures enable of reload reg_raw_num_total to internal cnt. 0: disable, 1: enable. valid only when dsi_bridge as flow controller"]
    #[inline(always)]
    #[must_use]
    pub fn raw_num_total_set(&mut self) -> RAW_NUM_TOTAL_SET_W<RAW_NUM_CFG_SPEC> {
        RAW_NUM_TOTAL_SET_W::new(self, 31)
    }
}
#[doc = "dsi bridge raw number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`raw_num_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`raw_num_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RAW_NUM_CFG_SPEC;
impl crate::RegisterSpec for RAW_NUM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`raw_num_cfg::R`](R) reader structure"]
impl crate::Readable for RAW_NUM_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`raw_num_cfg::W`](W) writer structure"]
impl crate::Writable for RAW_NUM_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets RAW_NUM_CFG to value 0x0003_8400"]
impl crate::Resettable for RAW_NUM_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0003_8400;
}
