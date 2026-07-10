#[doc = "Register `RGB2GRAY` reader"]
pub type R = crate::R<RGB2GRAY_SPEC>;
#[doc = "Register `RGB2GRAY` writer"]
pub type W = crate::W<RGB2GRAY_SPEC>;
#[doc = "Field `B` reader - Configures the b parameter for rgb2gray"]
pub type B_R = crate::FieldReader;
#[doc = "Field `B` writer - Configures the b parameter for rgb2gray"]
pub type B_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `G` reader - Configures the g parameter for rgb2gray"]
pub type G_R = crate::FieldReader;
#[doc = "Field `G` writer - Configures the g parameter for rgb2gray"]
pub type G_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `R` reader - Configures the r parameter for rgb2gray"]
pub type R_R = crate::FieldReader;
#[doc = "Field `R` writer - Configures the r parameter for rgb2gray"]
pub type R_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - Configures the b parameter for rgb2gray"]
    #[inline(always)]
    pub fn b(&self) -> B_R {
        B_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - Configures the g parameter for rgb2gray"]
    #[inline(always)]
    pub fn g(&self) -> G_R {
        G_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - Configures the r parameter for rgb2gray"]
    #[inline(always)]
    pub fn r(&self) -> R_R {
        R_R::new(((self.bits >> 16) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("RGB2GRAY")
            .field("b", &self.b())
            .field("g", &self.g())
            .field("r", &self.r())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - Configures the b parameter for rgb2gray"]
    #[inline(always)]
    pub fn b(&mut self) -> B_W<'_, RGB2GRAY_SPEC> {
        B_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - Configures the g parameter for rgb2gray"]
    #[inline(always)]
    pub fn g(&mut self) -> G_W<'_, RGB2GRAY_SPEC> {
        G_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - Configures the r parameter for rgb2gray"]
    #[inline(always)]
    pub fn r(&mut self) -> R_W<'_, RGB2GRAY_SPEC> {
        R_W::new(self, 16)
    }
}
#[doc = "rgb2gray register\n\nYou can [`read`](crate::Reg::read) this register and get [`rgb2gray::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`rgb2gray::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct RGB2GRAY_SPEC;
impl crate::RegisterSpec for RGB2GRAY_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`rgb2gray::R`](R) reader structure"]
impl crate::Readable for RGB2GRAY_SPEC {}
#[doc = "`write(|w| ..)` method takes [`rgb2gray::W`](W) writer structure"]
impl crate::Writable for RGB2GRAY_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets RGB2GRAY to value 0x0055_5655"]
impl crate::Resettable for RGB2GRAY_SPEC {
    const RESET_VALUE: u32 = 0x0055_5655;
}
