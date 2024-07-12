#[doc = "Register `WEIGHT_EN` reader"]
pub type R = crate::R<WEIGHT_EN_SPEC>;
#[doc = "Register `WEIGHT_EN` writer"]
pub type W = crate::W<WEIGHT_EN_SPEC>;
#[doc = "Field `TX` reader - This register is used to config tx arbiter weight function off/on"]
pub type TX_R = crate::BitReader;
#[doc = "Field `TX` writer - This register is used to config tx arbiter weight function off/on"]
pub type TX_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `RX` reader - This register is used to config rx arbiter weight function off/on"]
pub type RX_R = crate::BitReader;
#[doc = "Field `RX` writer - This register is used to config rx arbiter weight function off/on"]
pub type RX_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0 - This register is used to config tx arbiter weight function off/on"]
    #[inline(always)]
    pub fn tx(&self) -> TX_R {
        TX_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - This register is used to config rx arbiter weight function off/on"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new(((self.bits >> 1) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("WEIGHT_EN")
            .field("tx", &self.tx())
            .field("rx", &self.rx())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - This register is used to config tx arbiter weight function off/on"]
    #[inline(always)]
    #[must_use]
    pub fn tx(&mut self) -> TX_W<WEIGHT_EN_SPEC> {
        TX_W::new(self, 0)
    }
    #[doc = "Bit 1 - This register is used to config rx arbiter weight function off/on"]
    #[inline(always)]
    #[must_use]
    pub fn rx(&mut self) -> RX_W<WEIGHT_EN_SPEC> {
        RX_W::new(self, 1)
    }
}
#[doc = "This register is used to config arbiter weight function to on or off\n\nYou can [`read`](crate::Reg::read) this register and get [`weight_en::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`weight_en::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct WEIGHT_EN_SPEC;
impl crate::RegisterSpec for WEIGHT_EN_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`weight_en::R`](R) reader structure"]
impl crate::Readable for WEIGHT_EN_SPEC {}
#[doc = "`write(|w| ..)` method takes [`weight_en::W`](W) writer structure"]
impl crate::Writable for WEIGHT_EN_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets WEIGHT_EN to value 0"]
impl crate::Resettable for WEIGHT_EN_SPEC {
    const RESET_VALUE: u32 = 0;
}
