#[doc = "Register `CLK` reader"]
pub type R = crate::R<CLK_SPEC>;
#[doc = "Register `CLK` writer"]
pub type W = crate::W<CLK_SPEC>;
#[doc = "Field `SEL0` reader - "]
pub type SEL0_R = crate::FieldReader;
#[doc = "Field `SEL0` writer - "]
pub type SEL0_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `SEL1` reader - "]
pub type SEL1_R = crate::FieldReader;
#[doc = "Field `SEL1` writer - "]
pub type SEL1_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `EN` reader - "]
pub type EN_R = crate::BitReader;
#[doc = "Field `EN` writer - "]
pub type EN_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    pub fn sel0(&self) -> SEL0_R {
        SEL0_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    pub fn sel1(&self) -> SEL1_R {
        SEL1_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    pub fn en(&self) -> EN_R {
        EN_R::new(((self.bits >> 16) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CLK")
            .field("sel0", &self.sel0())
            .field("sel1", &self.sel1())
            .field("en", &self.en())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7"]
    #[inline(always)]
    #[must_use]
    pub fn sel0(&mut self) -> SEL0_W<CLK_SPEC> {
        SEL0_W::new(self, 0)
    }
    #[doc = "Bits 8:15"]
    #[inline(always)]
    #[must_use]
    pub fn sel1(&mut self) -> SEL1_W<CLK_SPEC> {
        SEL1_W::new(self, 8)
    }
    #[doc = "Bit 16"]
    #[inline(always)]
    #[must_use]
    pub fn en(&mut self) -> EN_W<CLK_SPEC> {
        EN_W::new(self, 16)
    }
}
#[doc = "\n\nYou can [`read`](crate::generic::Reg::read) this register and get [`clk::R`](R).  You can [`reset`](crate::generic::Reg::reset), [`write`](crate::generic::Reg::write), [`write_with_zero`](crate::generic::Reg::write_with_zero) this register using [`clk::W`](W). You can also [`modify`](crate::generic::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CLK_SPEC;
impl crate::RegisterSpec for CLK_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`clk::R`](R) reader structure"]
impl crate::Readable for CLK_SPEC {}
#[doc = "`write(|w| ..)` method takes [`clk::W`](W) writer structure"]
impl crate::Writable for CLK_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CLK to value 0x4052"]
impl crate::Resettable for CLK_SPEC {
    const RESET_VALUE: u32 = 0x4052;
}
