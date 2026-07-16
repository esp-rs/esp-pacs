#[doc = "Register `RD_MAC_SYS3` reader"]
pub type R = crate::R<RD_MAC_SYS3_SPEC>;
#[doc = "Register `RD_MAC_SYS3` writer"]
pub type W = crate::W<RD_MAC_SYS3_SPEC>;
#[doc = "Field `MAC_RESERVED_2` reader - Reserved."]
pub type MAC_RESERVED_2_R = crate::FieldReader<u32>;
#[doc = "Field `WAFER_VERSION_MINOR` reader - Represents the first 14-bit of zeroth part of system data."]
pub type WAFER_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `WAFER_VERSION_MAJOR` reader - "]
pub type WAFER_VERSION_MAJOR_R = crate::FieldReader;
#[doc = "Field `WAFER_VERSION_MAJOR` writer - "]
pub type WAFER_VERSION_MAJOR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DISABLE_WAFER_VERSION_MAJOR` reader - "]
pub type DISABLE_WAFER_VERSION_MAJOR_R = crate::BitReader;
#[doc = "Field `DISABLE_WAFER_VERSION_MAJOR` writer - "]
pub type DISABLE_WAFER_VERSION_MAJOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DISABLE_BLK_VERSION_MAJOR` reader - "]
pub type DISABLE_BLK_VERSION_MAJOR_R = crate::BitReader;
#[doc = "Field `DISABLE_BLK_VERSION_MAJOR` writer - "]
pub type DISABLE_BLK_VERSION_MAJOR_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `BLK_VERSION_MINOR` reader - "]
pub type BLK_VERSION_MINOR_R = crate::FieldReader;
#[doc = "Field `BLK_VERSION_MINOR` writer - "]
pub type BLK_VERSION_MINOR_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `BLK_VERSION_MAJOR` reader - "]
pub type BLK_VERSION_MAJOR_R = crate::FieldReader;
#[doc = "Field `BLK_VERSION_MAJOR` writer - "]
pub type BLK_VERSION_MAJOR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSRAM_CAP` reader - "]
pub type PSRAM_CAP_R = crate::BitReader;
#[doc = "Field `PSRAM_CAP` writer - "]
pub type PSRAM_CAP_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:17 - Reserved."]
    #[inline(always)]
    pub fn mac_reserved_2(&self) -> MAC_RESERVED_2_R {
        MAC_RESERVED_2_R::new(self.bits & 0x0003_ffff)
    }
    #[doc = "Bits 18:21 - Represents the first 14-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn wafer_version_minor(&self) -> WAFER_VERSION_MINOR_R {
        WAFER_VERSION_MINOR_R::new(((self.bits >> 18) & 0x0f) as u8)
    }
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn wafer_version_major(&self) -> WAFER_VERSION_MAJOR_R {
        WAFER_VERSION_MAJOR_R::new(((self.bits >> 22) & 3) as u8)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn disable_wafer_version_major(&self) -> DISABLE_WAFER_VERSION_MAJOR_R {
        DISABLE_WAFER_VERSION_MAJOR_R::new(((self.bits >> 24) & 1) != 0)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn disable_blk_version_major(&self) -> DISABLE_BLK_VERSION_MAJOR_R {
        DISABLE_BLK_VERSION_MAJOR_R::new(((self.bits >> 25) & 1) != 0)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn blk_version_minor(&self) -> BLK_VERSION_MINOR_R {
        BLK_VERSION_MINOR_R::new(((self.bits >> 26) & 7) as u8)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn blk_version_major(&self) -> BLK_VERSION_MAJOR_R {
        BLK_VERSION_MAJOR_R::new(((self.bits >> 29) & 3) as u8)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn psram_cap(&self) -> PSRAM_CAP_R {
        PSRAM_CAP_R::new(((self.bits >> 31) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS3")
            .field("mac_reserved_2", &self.mac_reserved_2())
            .field("wafer_version_minor", &self.wafer_version_minor())
            .field("wafer_version_major", &self.wafer_version_major())
            .field(
                "disable_wafer_version_major",
                &self.disable_wafer_version_major(),
            )
            .field(
                "disable_blk_version_major",
                &self.disable_blk_version_major(),
            )
            .field("blk_version_minor", &self.blk_version_minor())
            .field("blk_version_major", &self.blk_version_major())
            .field("psram_cap", &self.psram_cap())
            .finish()
    }
}
impl W {
    #[doc = "Bits 22:23"]
    #[inline(always)]
    pub fn wafer_version_major(&mut self) -> WAFER_VERSION_MAJOR_W<'_, RD_MAC_SYS3_SPEC> {
        WAFER_VERSION_MAJOR_W::new(self, 22)
    }
    #[doc = "Bit 24"]
    #[inline(always)]
    pub fn disable_wafer_version_major(
        &mut self,
    ) -> DISABLE_WAFER_VERSION_MAJOR_W<'_, RD_MAC_SYS3_SPEC> {
        DISABLE_WAFER_VERSION_MAJOR_W::new(self, 24)
    }
    #[doc = "Bit 25"]
    #[inline(always)]
    pub fn disable_blk_version_major(
        &mut self,
    ) -> DISABLE_BLK_VERSION_MAJOR_W<'_, RD_MAC_SYS3_SPEC> {
        DISABLE_BLK_VERSION_MAJOR_W::new(self, 25)
    }
    #[doc = "Bits 26:28"]
    #[inline(always)]
    pub fn blk_version_minor(&mut self) -> BLK_VERSION_MINOR_W<'_, RD_MAC_SYS3_SPEC> {
        BLK_VERSION_MINOR_W::new(self, 26)
    }
    #[doc = "Bits 29:30"]
    #[inline(always)]
    pub fn blk_version_major(&mut self) -> BLK_VERSION_MAJOR_W<'_, RD_MAC_SYS3_SPEC> {
        BLK_VERSION_MAJOR_W::new(self, 29)
    }
    #[doc = "Bit 31"]
    #[inline(always)]
    pub fn psram_cap(&mut self) -> PSRAM_CAP_W<'_, RD_MAC_SYS3_SPEC> {
        PSRAM_CAP_W::new(self, 31)
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys3::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_mac_sys3::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS3_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS3_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys3::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS3_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_mac_sys3::W`](W) writer structure"]
impl crate::Writable for RD_MAC_SYS3_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_MAC_SYS3 to value 0"]
impl crate::Resettable for RD_MAC_SYS3_SPEC {}
