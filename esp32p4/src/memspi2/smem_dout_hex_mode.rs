#[doc = "Register `SMEM_DOUT_HEX_MODE` reader"]
pub type R = crate::R<SMEM_DOUT_HEX_MODE_SPEC>;
#[doc = "Register `SMEM_DOUT_HEX_MODE` writer"]
pub type W = crate::W<SMEM_DOUT_HEX_MODE_SPEC>;
#[doc = "Field `DOUT_MODE(8-15)` reader - "]
pub type DOUT_MODE_R = crate::BitReader;
#[doc = "Field `DOUT_MODE(8-15)` writer - "]
pub type DOUT_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `DOUTS_HEX_MODE` reader - "]
pub type DOUTS_HEX_MODE_R = crate::BitReader;
#[doc = "Field `DOUTS_HEX_MODE` writer - "]
pub type DOUTS_HEX_MODE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT8_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&self, n: u8) -> DOUT_MODE_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DOUT_MODE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn dout_mode_iter(&self) -> impl Iterator<Item = DOUT_MODE_R> + '_ {
        (0..8).map(move |n| DOUT_MODE_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - DOUT8_MODE"]
    #[inline(always)]
    pub fn dout8_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - DOUT9_MODE"]
    #[inline(always)]
    pub fn dout9_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - DOUT10_MODE"]
    #[inline(always)]
    pub fn dout10_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - DOUT11_MODE"]
    #[inline(always)]
    pub fn dout11_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - DOUT12_MODE"]
    #[inline(always)]
    pub fn dout12_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - DOUT13_MODE"]
    #[inline(always)]
    pub fn dout13_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - DOUT14_MODE"]
    #[inline(always)]
    pub fn dout14_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - DOUT15_MODE"]
    #[inline(always)]
    pub fn dout15_mode(&self) -> DOUT_MODE_R {
        DOUT_MODE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn douts_hex_mode(&self) -> DOUTS_HEX_MODE_R {
        DOUTS_HEX_MODE_R::new(((self.bits >> 8) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DOUT_HEX_MODE")
            .field("dout8_mode", &self.dout8_mode())
            .field("dout9_mode", &self.dout9_mode())
            .field("dout10_mode", &self.dout10_mode())
            .field("dout11_mode", &self.dout11_mode())
            .field("dout12_mode", &self.dout12_mode())
            .field("dout13_mode", &self.dout13_mode())
            .field("dout14_mode", &self.dout14_mode())
            .field("dout15_mode", &self.dout15_mode())
            .field("douts_hex_mode", &self.douts_hex_mode())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DOUT8_MODE` field.</div>"]
    #[inline(always)]
    pub fn dout_mode(&mut self, n: u8) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DOUT_MODE_W::new(self, n)
    }
    #[doc = "Bit 0 - DOUT8_MODE"]
    #[inline(always)]
    pub fn dout8_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 0)
    }
    #[doc = "Bit 1 - DOUT9_MODE"]
    #[inline(always)]
    pub fn dout9_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 1)
    }
    #[doc = "Bit 2 - DOUT10_MODE"]
    #[inline(always)]
    pub fn dout10_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 2)
    }
    #[doc = "Bit 3 - DOUT11_MODE"]
    #[inline(always)]
    pub fn dout11_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 3)
    }
    #[doc = "Bit 4 - DOUT12_MODE"]
    #[inline(always)]
    pub fn dout12_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 4)
    }
    #[doc = "Bit 5 - DOUT13_MODE"]
    #[inline(always)]
    pub fn dout13_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 5)
    }
    #[doc = "Bit 6 - DOUT14_MODE"]
    #[inline(always)]
    pub fn dout14_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 6)
    }
    #[doc = "Bit 7 - DOUT15_MODE"]
    #[inline(always)]
    pub fn dout15_mode(&mut self) -> DOUT_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUT_MODE_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn douts_hex_mode(&mut self) -> DOUTS_HEX_MODE_W<'_, SMEM_DOUT_HEX_MODE_SPEC> {
        DOUTS_HEX_MODE_W::new(self, 8)
    }
}
#[doc = "\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_dout_hex_mode::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_dout_hex_mode::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DOUT_HEX_MODE_SPEC;
impl crate::RegisterSpec for SMEM_DOUT_HEX_MODE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_dout_hex_mode::R`](R) reader structure"]
impl crate::Readable for SMEM_DOUT_HEX_MODE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_dout_hex_mode::W`](W) writer structure"]
impl crate::Writable for SMEM_DOUT_HEX_MODE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_DOUT_HEX_MODE to value 0"]
impl crate::Resettable for SMEM_DOUT_HEX_MODE_SPEC {}
