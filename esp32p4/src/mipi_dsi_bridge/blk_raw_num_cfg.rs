#[doc = "Register `BLK_RAW_NUM_CFG` reader"]
pub type R = crate::R<BLK_RAW_NUM_CFG_SPEC>;
#[doc = "Register `BLK_RAW_NUM_CFG` writer"]
pub type W = crate::W<BLK_RAW_NUM_CFG_SPEC>;
#[doc = "Field `BLK_RAW_NUM_TOTAL` reader - this field configures number of total block pix bits/64"]
pub type BLK_RAW_NUM_TOTAL_R = crate::FieldReader<u32>;
#[doc = "Field `BLK_RAW_NUM_TOTAL` writer - this field configures number of total block pix bits/64"]
pub type BLK_RAW_NUM_TOTAL_W<'a, REG> = crate::FieldWriter<'a, REG, 22, u32>;
#[doc = "Field `BLK_RAW_NUM_TOTAL_SET` writer - write 1 to reload reg_blk_raw_num_total to internal cnt"]
pub type BLK_RAW_NUM_TOTAL_SET_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:21 - this field configures number of total block pix bits/64"]
    #[inline(always)]
    pub fn blk_raw_num_total(&self) -> BLK_RAW_NUM_TOTAL_R {
        BLK_RAW_NUM_TOTAL_R::new(self.bits & 0x003f_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLK_RAW_NUM_CFG")
            .field("blk_raw_num_total", &self.blk_raw_num_total().bits())
            .finish()
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for crate::generic::Reg<BLK_RAW_NUM_CFG_SPEC> {
    fn fmt(&self, f: &mut core::fmt::Formatter<'_>) -> core::fmt::Result {
        core::fmt::Debug::fmt(&self.read(), f)
    }
}
impl W {
    #[doc = "Bits 0:21 - this field configures number of total block pix bits/64"]
    #[inline(always)]
    #[must_use]
    pub fn blk_raw_num_total(&mut self) -> BLK_RAW_NUM_TOTAL_W<BLK_RAW_NUM_CFG_SPEC> {
        BLK_RAW_NUM_TOTAL_W::new(self, 0)
    }
    #[doc = "Bit 31 - write 1 to reload reg_blk_raw_num_total to internal cnt"]
    #[inline(always)]
    #[must_use]
    pub fn blk_raw_num_total_set(&mut self) -> BLK_RAW_NUM_TOTAL_SET_W<BLK_RAW_NUM_CFG_SPEC> {
        BLK_RAW_NUM_TOTAL_SET_W::new(self, 31)
    }
}
#[doc = "dsi_bridge block raw number control register\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`blk_raw_num_cfg::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`blk_raw_num_cfg::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLK_RAW_NUM_CFG_SPEC;
impl crate::RegisterSpec for BLK_RAW_NUM_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blk_raw_num_cfg::R`](R) reader structure"]
impl crate::Readable for BLK_RAW_NUM_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blk_raw_num_cfg::W`](W) writer structure"]
impl crate::Writable for BLK_RAW_NUM_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLK_RAW_NUM_CFG to value 0x0003_8400"]
impl crate::Resettable for BLK_RAW_NUM_CFG_SPEC {
    const RESET_VALUE: u32 = 0x0003_8400;
}
