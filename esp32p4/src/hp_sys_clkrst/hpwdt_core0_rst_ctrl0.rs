#[doc = "Register `HPWDT_CORE0_RST_CTRL0` reader"]
pub type R = crate::R<HPWDT_CORE0_RST_CTRL0_SPEC>;
#[doc = "Register `HPWDT_CORE0_RST_CTRL0` writer"]
pub type W = crate::W<HPWDT_CORE0_RST_CTRL0_SPEC>;
#[doc = "Field `HPCORE0_STALL_EN` reader - Reserved"]
pub type HPCORE0_STALL_EN_R = crate::BitReader;
#[doc = "Field `HPCORE0_STALL_EN` writer - Reserved"]
pub type HPCORE0_STALL_EN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `HPCORE0_STALL_WAIT_NUM` reader - Reserved"]
pub type HPCORE0_STALL_WAIT_NUM_R = crate::FieldReader;
#[doc = "Field `HPCORE0_STALL_WAIT_NUM` writer - Reserved"]
pub type HPCORE0_STALL_WAIT_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `WDT_HPCORE0_RST_LEN` reader - Reserved"]
pub type WDT_HPCORE0_RST_LEN_R = crate::FieldReader;
#[doc = "Field `WDT_HPCORE0_RST_LEN` writer - Reserved"]
pub type WDT_HPCORE0_RST_LEN_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    pub fn hpcore0_stall_en(&self) -> HPCORE0_STALL_EN_R {
        HPCORE0_STALL_EN_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bits 1:8 - Reserved"]
    #[inline(always)]
    pub fn hpcore0_stall_wait_num(&self) -> HPCORE0_STALL_WAIT_NUM_R {
        HPCORE0_STALL_WAIT_NUM_R::new(((self.bits >> 1) & 0xff) as u8)
    }
    #[doc = "Bits 9:16 - Reserved"]
    #[inline(always)]
    pub fn wdt_hpcore0_rst_len(&self) -> WDT_HPCORE0_RST_LEN_R {
        WDT_HPCORE0_RST_LEN_R::new(((self.bits >> 9) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("HPWDT_CORE0_RST_CTRL0")
            .field("hpcore0_stall_en", &self.hpcore0_stall_en())
            .field("hpcore0_stall_wait_num", &self.hpcore0_stall_wait_num())
            .field("wdt_hpcore0_rst_len", &self.wdt_hpcore0_rst_len())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn hpcore0_stall_en(&mut self) -> HPCORE0_STALL_EN_W<HPWDT_CORE0_RST_CTRL0_SPEC> {
        HPCORE0_STALL_EN_W::new(self, 0)
    }
    #[doc = "Bits 1:8 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn hpcore0_stall_wait_num(
        &mut self,
    ) -> HPCORE0_STALL_WAIT_NUM_W<HPWDT_CORE0_RST_CTRL0_SPEC> {
        HPCORE0_STALL_WAIT_NUM_W::new(self, 1)
    }
    #[doc = "Bits 9:16 - Reserved"]
    #[inline(always)]
    #[must_use]
    pub fn wdt_hpcore0_rst_len(&mut self) -> WDT_HPCORE0_RST_LEN_W<HPWDT_CORE0_RST_CTRL0_SPEC> {
        WDT_HPCORE0_RST_LEN_W::new(self, 9)
    }
}
#[doc = "Reserved\n\nYou can [`read`](crate::Reg::read) this register and get [`hpwdt_core0_rst_ctrl0::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`hpwdt_core0_rst_ctrl0::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct HPWDT_CORE0_RST_CTRL0_SPEC;
impl crate::RegisterSpec for HPWDT_CORE0_RST_CTRL0_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`hpwdt_core0_rst_ctrl0::R`](R) reader structure"]
impl crate::Readable for HPWDT_CORE0_RST_CTRL0_SPEC {}
#[doc = "`write(|w| ..)` method takes [`hpwdt_core0_rst_ctrl0::W`](W) writer structure"]
impl crate::Writable for HPWDT_CORE0_RST_CTRL0_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets HPWDT_CORE0_RST_CTRL0 to value 0x1011"]
impl crate::Resettable for HPWDT_CORE0_RST_CTRL0_SPEC {
    const RESET_VALUE: u32 = 0x1011;
}
