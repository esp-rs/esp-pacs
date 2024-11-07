#[doc = "Register `GEN_VCID` reader"]
pub type R = crate::R<GEN_VCID_SPEC>;
#[doc = "Register `GEN_VCID` writer"]
pub type W = crate::W<GEN_VCID_SPEC>;
#[doc = "Field `RX` reader - NA"]
pub type RX_R = crate::FieldReader;
#[doc = "Field `RX` writer - NA"]
pub type RX_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TEAR_AUTO` reader - NA"]
pub type TEAR_AUTO_R = crate::FieldReader;
#[doc = "Field `TEAR_AUTO` writer - NA"]
pub type TEAR_AUTO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `TX_AUTO` reader - NA"]
pub type TX_AUTO_R = crate::FieldReader;
#[doc = "Field `TX_AUTO` writer - NA"]
pub type TX_AUTO_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
impl R {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn rx(&self) -> RX_R {
        RX_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 8:9 - NA"]
    #[inline(always)]
    pub fn tear_auto(&self) -> TEAR_AUTO_R {
        TEAR_AUTO_R::new(((self.bits >> 8) & 3) as u8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    pub fn tx_auto(&self) -> TX_AUTO_R {
        TX_AUTO_R::new(((self.bits >> 16) & 3) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("GEN_VCID")
            .field("rx", &self.rx())
            .field("tear_auto", &self.tear_auto())
            .field("tx_auto", &self.tx_auto())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1 - NA"]
    #[inline(always)]
    pub fn rx(&mut self) -> RX_W<GEN_VCID_SPEC> {
        RX_W::new(self, 0)
    }
    #[doc = "Bits 8:9 - NA"]
    #[inline(always)]
    pub fn tear_auto(&mut self) -> TEAR_AUTO_W<GEN_VCID_SPEC> {
        TEAR_AUTO_W::new(self, 8)
    }
    #[doc = "Bits 16:17 - NA"]
    #[inline(always)]
    pub fn tx_auto(&mut self) -> TX_AUTO_W<GEN_VCID_SPEC> {
        TX_AUTO_W::new(self, 16)
    }
}
#[doc = "NA\n\nYou can [`read`](crate::Reg::read) this register and get [`gen_vcid::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`gen_vcid::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct GEN_VCID_SPEC;
impl crate::RegisterSpec for GEN_VCID_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`gen_vcid::R`](R) reader structure"]
impl crate::Readable for GEN_VCID_SPEC {}
#[doc = "`write(|w| ..)` method takes [`gen_vcid::W`](W) writer structure"]
impl crate::Writable for GEN_VCID_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets GEN_VCID to value 0"]
impl crate::Resettable for GEN_VCID_SPEC {
    const RESET_VALUE: u32 = 0;
}
