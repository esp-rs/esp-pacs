#[doc = "Register `UHS` reader"]
pub type R = crate::R<UHS_SPEC>;
#[doc = "Register `UHS` writer"]
pub type W = crate::W<UHS_SPEC>;
#[doc = "Field `DDR` reader - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DDR_R = crate::FieldReader;
#[doc = "Field `DDR` writer - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
pub type DDR_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&self) -> DDR_R {
        DDR_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("UHS").field("ddr", &self.ddr()).finish()
    }
}
impl W {
    #[doc = "Bits 16:17 - DDR mode selecton,1 bit for each card. 0-Non-DDR mdoe. 1-DDR mdoe."]
    #[inline(always)]
    pub fn ddr(&mut self) -> DDR_W<UHS_SPEC> {
        DDR_W::new(self, 16)
    }
}
#[doc = "UHS-1 register\n\nYou can [`read`](crate::Reg::read) this register and get [`uhs::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`uhs::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct UHS_SPEC;
impl crate::RegisterSpec for UHS_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`uhs::R`](R) reader structure"]
impl crate::Readable for UHS_SPEC {}
#[doc = "`write(|w| ..)` method takes [`uhs::W`](W) writer structure"]
impl crate::Writable for UHS_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets UHS to value 0"]
impl crate::Resettable for UHS_SPEC {
    const RESET_VALUE: u32 = 0;
}
