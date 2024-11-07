#[doc = "Register `BLC_CTRL2` reader"]
pub type R = crate::R<BLC_CTRL2_SPEC>;
#[doc = "Register `BLC_CTRL2` writer"]
pub type W = crate::W<BLC_CTRL2_SPEC>;
#[doc = "Field `BLC_R3_TH` reader - this field configures black threshold when get blc average of bottom right channel"]
pub type BLC_R3_TH_R = crate::FieldReader;
#[doc = "Field `BLC_R3_TH` writer - this field configures black threshold when get blc average of bottom right channel"]
pub type BLC_R3_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R2_TH` reader - this field configures black threshold when get blc average of bottom left channel"]
pub type BLC_R2_TH_R = crate::FieldReader;
#[doc = "Field `BLC_R2_TH` writer - this field configures black threshold when get blc average of bottom left channel"]
pub type BLC_R2_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R1_TH` reader - this field configures black threshold when get blc average of top right channel"]
pub type BLC_R1_TH_R = crate::FieldReader;
#[doc = "Field `BLC_R1_TH` writer - this field configures black threshold when get blc average of top right channel"]
pub type BLC_R1_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
#[doc = "Field `BLC_R0_TH` reader - this field configures black threshold when get blc average of top left channel"]
pub type BLC_R0_TH_R = crate::FieldReader;
#[doc = "Field `BLC_R0_TH` writer - this field configures black threshold when get blc average of top left channel"]
pub type BLC_R0_TH_W<'a, REG> = crate::FieldWriter<'a, REG, 8>;
impl R {
    #[doc = "Bits 0:7 - this field configures black threshold when get blc average of bottom right channel"]
    #[inline(always)]
    pub fn blc_r3_th(&self) -> BLC_R3_TH_R {
        BLC_R3_TH_R::new((self.bits & 0xff) as u8)
    }
    #[doc = "Bits 8:15 - this field configures black threshold when get blc average of bottom left channel"]
    #[inline(always)]
    pub fn blc_r2_th(&self) -> BLC_R2_TH_R {
        BLC_R2_TH_R::new(((self.bits >> 8) & 0xff) as u8)
    }
    #[doc = "Bits 16:23 - this field configures black threshold when get blc average of top right channel"]
    #[inline(always)]
    pub fn blc_r1_th(&self) -> BLC_R1_TH_R {
        BLC_R1_TH_R::new(((self.bits >> 16) & 0xff) as u8)
    }
    #[doc = "Bits 24:31 - this field configures black threshold when get blc average of top left channel"]
    #[inline(always)]
    pub fn blc_r0_th(&self) -> BLC_R0_TH_R {
        BLC_R0_TH_R::new(((self.bits >> 24) & 0xff) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("BLC_CTRL2")
            .field("blc_r3_th", &self.blc_r3_th())
            .field("blc_r2_th", &self.blc_r2_th())
            .field("blc_r1_th", &self.blc_r1_th())
            .field("blc_r0_th", &self.blc_r0_th())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:7 - this field configures black threshold when get blc average of bottom right channel"]
    #[inline(always)]
    pub fn blc_r3_th(&mut self) -> BLC_R3_TH_W<BLC_CTRL2_SPEC> {
        BLC_R3_TH_W::new(self, 0)
    }
    #[doc = "Bits 8:15 - this field configures black threshold when get blc average of bottom left channel"]
    #[inline(always)]
    pub fn blc_r2_th(&mut self) -> BLC_R2_TH_W<BLC_CTRL2_SPEC> {
        BLC_R2_TH_W::new(self, 8)
    }
    #[doc = "Bits 16:23 - this field configures black threshold when get blc average of top right channel"]
    #[inline(always)]
    pub fn blc_r1_th(&mut self) -> BLC_R1_TH_W<BLC_CTRL2_SPEC> {
        BLC_R1_TH_W::new(self, 16)
    }
    #[doc = "Bits 24:31 - this field configures black threshold when get blc average of top left channel"]
    #[inline(always)]
    pub fn blc_r0_th(&mut self) -> BLC_R0_TH_W<BLC_CTRL2_SPEC> {
        BLC_R0_TH_W::new(self, 24)
    }
}
#[doc = "blc black threshold control register\n\nYou can [`read`](crate::Reg::read) this register and get [`blc_ctrl2::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`blc_ctrl2::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct BLC_CTRL2_SPEC;
impl crate::RegisterSpec for BLC_CTRL2_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`blc_ctrl2::R`](R) reader structure"]
impl crate::Readable for BLC_CTRL2_SPEC {}
#[doc = "`write(|w| ..)` method takes [`blc_ctrl2::W`](W) writer structure"]
impl crate::Writable for BLC_CTRL2_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets BLC_CTRL2 to value 0"]
impl crate::Resettable for BLC_CTRL2_SPEC {
    const RESET_VALUE: u32 = 0;
}
