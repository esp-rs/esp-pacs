#[doc = "Register `SMEM_DIN_HEX_MODE` reader"]
pub type R = crate::R<SMEM_DIN_HEX_MODE_SPEC>;
#[doc = "Register `SMEM_DIN_HEX_MODE` writer"]
pub type W = crate::W<SMEM_DIN_HEX_MODE_SPEC>;
#[doc = "Field `DIN_MODE(8-15)` reader - "]
pub type DIN_MODE_R = crate::FieldReader;
#[doc = "Field `DIN_MODE(8-15)` writer - "]
pub type DIN_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `DINS_HEX_MODE` reader - "]
pub type DINS_HEX_MODE_R = crate::FieldReader;
#[doc = "Field `DINS_HEX_MODE` writer - "]
pub type DINS_HEX_MODE_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN8_MODE` field.</div>"]
    #[inline(always)]
    pub fn din_mode(&self, n: u8) -> DIN_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_MODE_R::new(((self.bits >> (n * 3)) & 7) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn din_mode_iter(&self) -> impl Iterator<Item = DIN_MODE_R> + '_ {
        (0..8).map(move |n| DIN_MODE_R::new(((self.bits >> (n * 3)) & 7) as u8))
    }
    #[doc = "Bits 0:2 - DIN8_MODE"]
    #[inline(always)]
    pub fn din8_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bits 3:5 - DIN9_MODE"]
    #[inline(always)]
    pub fn din9_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 3) & 7) as u8)
    }
    #[doc = "Bits 6:8 - DIN10_MODE"]
    #[inline(always)]
    pub fn din10_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 6) & 7) as u8)
    }
    #[doc = "Bits 9:11 - DIN11_MODE"]
    #[inline(always)]
    pub fn din11_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 9) & 7) as u8)
    }
    #[doc = "Bits 12:14 - DIN12_MODE"]
    #[inline(always)]
    pub fn din12_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 12) & 7) as u8)
    }
    #[doc = "Bits 15:17 - DIN13_MODE"]
    #[inline(always)]
    pub fn din13_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 15) & 7) as u8)
    }
    #[doc = "Bits 18:20 - DIN14_MODE"]
    #[inline(always)]
    pub fn din14_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 18) & 7) as u8)
    }
    #[doc = "Bits 21:23 - DIN15_MODE"]
    #[inline(always)]
    pub fn din15_mode(&self) -> DIN_MODE_R {
        DIN_MODE_R::new(((self.bits >> 21) & 7) as u8)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dins_hex_mode(&self) -> DINS_HEX_MODE_R {
        DINS_HEX_MODE_R::new(((self.bits >> 24) & 7) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DIN_HEX_MODE")
            .field("din8_mode", &self.din8_mode())
            .field("din9_mode", &self.din9_mode())
            .field("din10_mode", &self.din10_mode())
            .field("din11_mode", &self.din11_mode())
            .field("din12_mode", &self.din12_mode())
            .field("din13_mode", &self.din13_mode())
            .field("din14_mode", &self.din14_mode())
            .field("din15_mode", &self.din15_mode())
            .field("dins_hex_mode", &self.dins_hex_mode())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN8_MODE` field.</div>"]
    #[inline(always)]
    pub fn din_mode(&mut self, n: u8) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_MODE_W::new(self, n * 3)
    }
    #[doc = "Bits 0:2 - DIN8_MODE"]
    #[inline(always)]
    pub fn din8_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 0)
    }
    #[doc = "Bits 3:5 - DIN9_MODE"]
    #[inline(always)]
    pub fn din9_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 3)
    }
    #[doc = "Bits 6:8 - DIN10_MODE"]
    #[inline(always)]
    pub fn din10_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 6)
    }
    #[doc = "Bits 9:11 - DIN11_MODE"]
    #[inline(always)]
    pub fn din11_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 9)
    }
    #[doc = "Bits 12:14 - DIN12_MODE"]
    #[inline(always)]
    pub fn din12_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 12)
    }
    #[doc = "Bits 15:17 - DIN13_MODE"]
    #[inline(always)]
    pub fn din13_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 15)
    }
    #[doc = "Bits 18:20 - DIN14_MODE"]
    #[inline(always)]
    pub fn din14_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 18)
    }
    #[doc = "Bits 21:23 - DIN15_MODE"]
    #[inline(always)]
    pub fn din15_mode(&mut self) -> DIN_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DIN_MODE_W::new(self, 21)
    }
    #[doc = "Bits 24:26"]
    #[inline(always)]
    pub fn dins_hex_mode(&mut self) -> DINS_HEX_MODE_W<'_, SMEM_DIN_HEX_MODE_SPEC> {
        DINS_HEX_MODE_W::new(self, 24)
    }
}
#[doc = "MSPI 16x external RAM input timing delay mode control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_hex_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DIN_HEX_MODE_SPEC;
impl crate::RegisterSpec for SMEM_DIN_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_din_hex_mode::R`](R) reader structure"]
impl crate::Readable for SMEM_DIN_HEX_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_din_hex_mode::W`](W) writer structure"]
impl crate::Writable for SMEM_DIN_HEX_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_DIN_HEX_MODE to value 0"]
impl crate::Resettable for SMEM_DIN_HEX_MODE_SPEC {}
