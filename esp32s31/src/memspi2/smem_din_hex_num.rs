#[doc = "Register `SMEM_DIN_HEX_NUM` reader"]
pub type R = crate::R<SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "Register `SMEM_DIN_HEX_NUM` writer"]
pub type W = crate::W<SMEM_DIN_HEX_NUM_SPEC>;
#[doc = "Field `DIN_NUM(8-15)` reader - "]
pub type DIN_NUM_R = crate::FieldReader;
#[doc = "Field `DIN_NUM(8-15)` writer - "]
pub type DIN_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `DINS_HEX_NUM` reader - "]
pub type DINS_HEX_NUM_R = crate::FieldReader;
#[doc = "Field `DINS_HEX_NUM` writer - "]
pub type DINS_HEX_NUM_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN8_NUM` field.</div>"]
    #[inline(always)]
    pub fn din_num(&self, n: u8) -> DIN_NUM_R {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_NUM_R::new(((self.bits >> (n * 2)) & 3) as u8)
    }
    #[doc = "Iterator for array of:"]
    #[doc = ""]
    #[inline(always)]
    pub fn din_num_iter(&self) -> impl Iterator<Item = DIN_NUM_R> + '_ {
        (0..8).map(move |n| DIN_NUM_R::new(((self.bits >> (n * 2)) & 3) as u8))
    }
    #[doc = "Bits 0:1 - DIN8_NUM"]
    #[inline(always)]
    pub fn din8_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:3 - DIN9_NUM"]
    #[inline(always)]
    pub fn din9_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 2) & 3) as u8)
    }
    #[doc = "Bits 4:5 - DIN10_NUM"]
    #[inline(always)]
    pub fn din10_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 4) & 3) as u8)
    }
    #[doc = "Bits 6:7 - DIN11_NUM"]
    #[inline(always)]
    pub fn din11_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:9 - DIN12_NUM"]
    #[inline(always)]
    pub fn din12_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 10:11 - DIN13_NUM"]
    #[inline(always)]
    pub fn din13_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 10) & 3) as u8)
    }
    #[doc = "Bits 12:13 - DIN14_NUM"]
    #[inline(always)]
    pub fn din14_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:15 - DIN15_NUM"]
    #[inline(always)]
    pub fn din15_num(&self) -> DIN_NUM_R {
        DIN_NUM_R::new(((self.bits >> 14) & 3) as u8)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dins_hex_num(&self) -> DINS_HEX_NUM_R {
        DINS_HEX_NUM_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SMEM_DIN_HEX_NUM")
            .field("din8_num", &self.din8_num())
            .field("din9_num", &self.din9_num())
            .field("din10_num", &self.din10_num())
            .field("din11_num", &self.din11_num())
            .field("din12_num", &self.din12_num())
            .field("din13_num", &self.din13_num())
            .field("din14_num", &self.din14_num())
            .field("din15_num", &self.din15_num())
            .field("dins_hex_num", &self.dins_hex_num())
            .finish()
    }
}
impl W {
    #[doc = ""]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `DIN8_NUM` field.</div>"]
    #[inline(always)]
    pub fn din_num(&mut self, n: u8) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 8][n as usize];
        DIN_NUM_W::new(self, n * 2)
    }
    #[doc = "Bits 0:1 - DIN8_NUM"]
    #[inline(always)]
    pub fn din8_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 0)
    }
    #[doc = "Bits 2:3 - DIN9_NUM"]
    #[inline(always)]
    pub fn din9_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 2)
    }
    #[doc = "Bits 4:5 - DIN10_NUM"]
    #[inline(always)]
    pub fn din10_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 4)
    }
    #[doc = "Bits 6:7 - DIN11_NUM"]
    #[inline(always)]
    pub fn din11_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 6)
    }
    #[doc = "Bits 8:9 - DIN12_NUM"]
    #[inline(always)]
    pub fn din12_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 8)
    }
    #[doc = "Bits 10:11 - DIN13_NUM"]
    #[inline(always)]
    pub fn din13_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 10)
    }
    #[doc = "Bits 12:13 - DIN14_NUM"]
    #[inline(always)]
    pub fn din14_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 12)
    }
    #[doc = "Bits 14:15 - DIN15_NUM"]
    #[inline(always)]
    pub fn din15_num(&mut self) -> DIN_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DIN_NUM_W::new(self, 14)
    }
    #[doc = "Bits 16:17"]
    #[inline(always)]
    pub fn dins_hex_num(&mut self) -> DINS_HEX_NUM_W<'_, SMEM_DIN_HEX_NUM_SPEC> {
        DINS_HEX_NUM_W::new(self, 16)
    }
}
#[doc = "MSPI 16x external RAM input timing delay number control register\n\nYou can [`read`](crate::Reg::read) this register and get [`smem_din_hex_num::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`smem_din_hex_num::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SMEM_DIN_HEX_NUM_SPEC;
impl crate::RegisterSpec for SMEM_DIN_HEX_NUM_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`smem_din_hex_num::R`](R) reader structure"]
impl crate::Readable for SMEM_DIN_HEX_NUM_SPEC {}
#[doc = "`write(|w| ..)` method takes [`smem_din_hex_num::W`](W) writer structure"]
impl crate::Writable for SMEM_DIN_HEX_NUM_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SMEM_DIN_HEX_NUM to value 0"]
impl crate::Resettable for SMEM_DIN_HEX_NUM_SPEC {}
