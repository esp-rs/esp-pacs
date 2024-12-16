#[doc = "Register `PAD_COMP1` reader"]
pub type R = crate::R<PAD_COMP1_SPEC>;
#[doc = "Register `PAD_COMP1` writer"]
pub type W = crate::W<PAD_COMP1_SPEC>;
#[doc = "Field `DREF_COMP1` reader - pad comp dref"]
pub type DREF_COMP1_R = crate::FieldReader;
#[doc = "Field `DREF_COMP1` writer - pad comp dref"]
pub type DREF_COMP1_W<'a, REG> = crate::FieldWriter<'a, REG, 3>;
#[doc = "Field `MODE_COMP1` reader - pad comp mode"]
pub type MODE_COMP1_R = crate::BitReader;
#[doc = "Field `MODE_COMP1` writer - pad comp mode"]
pub type MODE_COMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `XPD_COMP1` reader - pad comp xpd"]
pub type XPD_COMP1_R = crate::BitReader;
#[doc = "Field `XPD_COMP1` writer - pad comp xpd"]
pub type XPD_COMP1_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bits 0:2 - pad comp dref"]
    #[inline(always)]
    pub fn dref_comp1(&self) -> DREF_COMP1_R {
        DREF_COMP1_R::new((self.bits & 7) as u8)
    }
    #[doc = "Bit 3 - pad comp mode"]
    #[inline(always)]
    pub fn mode_comp1(&self) -> MODE_COMP1_R {
        MODE_COMP1_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - pad comp xpd"]
    #[inline(always)]
    pub fn xpd_comp1(&self) -> XPD_COMP1_R {
        XPD_COMP1_R::new(((self.bits >> 4) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("PAD_COMP1")
            .field("dref_comp1", &self.dref_comp1())
            .field("mode_comp1", &self.mode_comp1())
            .field("xpd_comp1", &self.xpd_comp1())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:2 - pad comp dref"]
    #[inline(always)]
    pub fn dref_comp1(&mut self) -> DREF_COMP1_W<PAD_COMP1_SPEC> {
        DREF_COMP1_W::new(self, 0)
    }
    #[doc = "Bit 3 - pad comp mode"]
    #[inline(always)]
    pub fn mode_comp1(&mut self) -> MODE_COMP1_W<PAD_COMP1_SPEC> {
        MODE_COMP1_W::new(self, 3)
    }
    #[doc = "Bit 4 - pad comp xpd"]
    #[inline(always)]
    pub fn xpd_comp1(&mut self) -> XPD_COMP1_W<PAD_COMP1_SPEC> {
        XPD_COMP1_W::new(self, 4)
    }
}
#[doc = "need_des\n\nYou can [`read`](crate::Reg::read) this register and get [`pad_comp1::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`pad_comp1::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct PAD_COMP1_SPEC;
impl crate::RegisterSpec for PAD_COMP1_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`pad_comp1::R`](R) reader structure"]
impl crate::Readable for PAD_COMP1_SPEC {}
#[doc = "`write(|w| ..)` method takes [`pad_comp1::W`](W) writer structure"]
impl crate::Writable for PAD_COMP1_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets PAD_COMP1 to value 0"]
impl crate::Resettable for PAD_COMP1_SPEC {
    const RESET_VALUE: u32 = 0;
}
