#[doc = "Register `RD_MAC_SYS4` reader"]
pub type R = crate::R<RD_MAC_SYS4_SPEC>;
#[doc = "Register `RD_MAC_SYS4` writer"]
pub type W = crate::W<RD_MAC_SYS4_SPEC>;
#[doc = "Field `PSRAM_CAP_1` reader - Represents the second 32-bit of zeroth part of system data."]
pub type PSRAM_CAP_1_R = crate::FieldReader;
#[doc = "Field `TEMP` reader - "]
pub type TEMP_R = crate::FieldReader;
#[doc = "Field `TEMP` writer - "]
pub type TEMP_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PSRAM_VENDOR` reader - "]
pub type PSRAM_VENDOR_R = crate::FieldReader;
#[doc = "Field `PSRAM_VENDOR` writer - "]
pub type PSRAM_VENDOR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `PKG_VERSION` reader - "]
pub type PKG_VERSION_R = crate::FieldReader;
#[doc = "Field `PKG_VERSION` writer - "]
pub type PKG_VERSION_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `RESERVED_1_136` reader - "]
pub type RESERVED_1_136_R = crate::FieldReader<u32>;
#[doc = "Field `RESERVED_1_136` writer - "]
pub type RESERVED_1_136_W<'a, REG> = crate::FieldWriter<'a, REG, 24, u32>;
impl R {
    #[doc = "Bits 0:1 - Represents the second 32-bit of zeroth part of system data."]
    #[inline(always)]
    pub fn psram_cap_1(&self) -> PSRAM_CAP_1_R {
        PSRAM_CAP_1_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn temp(&self) -> TEMP_R {
        TEMP_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn psram_vendor(&self) -> PSRAM_VENDOR_R {
        PSRAM_VENDOR_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pkg_version(&self) -> PKG_VERSION_R {
        PKG_VERSION_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved_1_136(&self) -> RESERVED_1_136_R {
        RESERVED_1_136_R::new((self.bits >> 8) & 0x00ff_ffff)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RD_MAC_SYS4")
            .field("psram_cap_1", &self.psram_cap_1())
            .field("temp", &self.temp())
            .field("psram_vendor", &self.psram_vendor())
            .field("pkg_version", &self.pkg_version())
            .field("reserved_1_136", &self.reserved_1_136())
            .finish()
    }
}
impl W {
    #[doc = "Bits 2:3"]
    #[inline(always)]
    pub fn temp(&mut self) -> TEMP_W<'_, RD_MAC_SYS4_SPEC> {
        TEMP_W::new(self, 2)
    }
    #[doc = "Bits 4:5"]
    #[inline(always)]
    pub fn psram_vendor(&mut self) -> PSRAM_VENDOR_W<'_, RD_MAC_SYS4_SPEC> {
        PSRAM_VENDOR_W::new(self, 4)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn pkg_version(&mut self) -> PKG_VERSION_W<'_, RD_MAC_SYS4_SPEC> {
        PKG_VERSION_W::new(self, 6)
    }
    #[doc = "Bits 8:31"]
    #[inline(always)]
    pub fn reserved_1_136(&mut self) -> RESERVED_1_136_W<'_, RD_MAC_SYS4_SPEC> {
        RESERVED_1_136_W::new(self, 8)
    }
}
#[doc = "Represents rd_mac_sys\n\nYou can [`read`](crate::Reg::read) this register and get [`rd_mac_sys4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rd_mac_sys4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RD_MAC_SYS4_SPEC;
impl crate::RegisterSpec for RD_MAC_SYS4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rd_mac_sys4::R`](R) reader structure"]
impl crate::Readable for RD_MAC_SYS4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rd_mac_sys4::W`](W) writer structure"]
impl crate::Writable for RD_MAC_SYS4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RD_MAC_SYS4 to value 0"]
impl crate::Resettable for RD_MAC_SYS4_SPEC {}
