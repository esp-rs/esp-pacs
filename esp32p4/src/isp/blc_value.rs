#[doc = "Register `BLC_VALUE` reader"]
pub type R = crate::R<BLC_VALUE_SPEC>;
#[doc = "Register `BLC_VALUE` writer"]
pub type W = crate::W<BLC_VALUE_SPEC>;
#[doc = "Field `BLC_R3_VALUE` reader - this field configures the black level of bottom right channel of bayer img"]
pub type BLC_R3_VALUE_R = crate::FieldReader;
#[doc = "Field `BLC_R3_VALUE` writer - this field configures the black level of bottom right channel of bayer img"]
pub type BLC_R3_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R2_VALUE` reader - this field configures the black level of bottom left channel of bayer img"]
pub type BLC_R2_VALUE_R = crate::FieldReader;
#[doc = "Field `BLC_R2_VALUE` writer - this field configures the black level of bottom left channel of bayer img"]
pub type BLC_R2_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R1_VALUE` reader - this field configures the black level of top right channel of bayer img"]
pub type BLC_R1_VALUE_R = crate::FieldReader;
#[doc = "Field `BLC_R1_VALUE` writer - this field configures the black level of top right channel of bayer img"]
pub type BLC_R1_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R0_VALUE` reader - this field configures the black level of top left channel of bayer img"]
pub type BLC_R0_VALUE_R = crate::FieldReader;
#[doc = "Field `BLC_R0_VALUE` writer - this field configures the black level of top left channel of bayer img"]
pub type BLC_R0_VALUE_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures the black level of bottom right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r3_value(&self) -> BLC_R3_VALUE_R {
        BLC_R3_VALUE_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures the black level of bottom left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r2_value(&self) -> BLC_R2_VALUE_R {
        BLC_R2_VALUE_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures the black level of top right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r1_value(&self) -> BLC_R1_VALUE_R {
        BLC_R1_VALUE_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures the black level of top left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r0_value(&self) -> BLC_R0_VALUE_R {
        BLC_R0_VALUE_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLC_VALUE")
            .field("blc_r3_value", &self.blc_r3_value())
            .field("blc_r2_value", &self.blc_r2_value())
            .field("blc_r1_value", &self.blc_r1_value())
            .field("blc_r0_value", &self.blc_r0_value())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures the black level of bottom right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r3_value(&mut self) -> BLC_R3_VALUE_W<BLC_VALUE_SPEC> {
        BLC_R3_VALUE_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures the black level of bottom left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r2_value(&mut self) -> BLC_R2_VALUE_W<BLC_VALUE_SPEC> {
        BLC_R2_VALUE_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures the black level of top right channel of bayer img"]
    #[inline(always)]
    pub fn blc_r1_value(&mut self) -> BLC_R1_VALUE_W<BLC_VALUE_SPEC> {
        BLC_R1_VALUE_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures the black level of top left channel of bayer img"]
    #[inline(always)]
    pub fn blc_r0_value(&mut self) -> BLC_R0_VALUE_W<BLC_VALUE_SPEC> {
        BLC_R0_VALUE_W::new(self, 24)
    }
}
#[doc = "blc black level register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_value::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_value::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLC_VALUE_SPEC;
impl crate::RegisterSpec for BLC_VALUE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blc_value::R`](R) reader structure"]
impl crate::Readable for BLC_VALUE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blc_value::W`](W) writer structure"]
impl crate::Writable for BLC_VALUE_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLC_VALUE to value 0"]
impl crate::Resettable for BLC_VALUE_SPEC {
    const RESET_VALUE: u32 = 0;
}
