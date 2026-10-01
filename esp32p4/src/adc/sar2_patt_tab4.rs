#[doc = "Register `SAR2_PATT_TAB4` reader"]
pub type R = crate::R<SAR2_PATT_TAB4_SPEC>;
#[doc = "Register `SAR2_PATT_TAB4` writer"]
pub type W = crate::W<SAR2_PATT_TAB4_SPEC>;
#[doc = "\n\nValue on reset: 0"]
#[cfg_attr(feature = "defmt", derive(defmt::Format))]
#[derive(Clone, Copy, Debug, PartialEq, Eq)]
#[repr(u8)]
pub enum ITEM15_ATTEN {
    #[doc = "0: No input attenuation, ADC can measure up to approx."]
    Db0   = 0,
    #[doc = "1: The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    Db2_5 = 1,
    #[doc = "2: The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    Db6   = 2,
    #[doc = "3: The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    Db12  = 3,
}
impl From<ITEM15_ATTEN> for u8 {
    #[inline(always)]
    fn from(variant: ITEM15_ATTEN) -> Self {
        variant as _
    }
}
impl crate::FieldSpec for ITEM15_ATTEN {
    type Ux = u8;
}
impl crate::IsEnum for ITEM15_ATTEN {}
#[doc = "Field `ITEM15_ATTEN` reader - "]
pub type ITEM15_ATTEN_R = crate::FieldReader<ITEM15_ATTEN>;
impl ITEM15_ATTEN_R {
    #[doc = "Get enumerated values variant"]
    #[inline(always)]
    pub const fn variant(&self) -> ITEM15_ATTEN {
        match self.bits {
            0 => ITEM15_ATTEN::Db0,
            1 => ITEM15_ATTEN::Db2_5,
            2 => ITEM15_ATTEN::Db6,
            3 => ITEM15_ATTEN::Db12,
            _ => unreachable!(),
        }
    }
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn is_db0(&self) -> bool {
        *self == ITEM15_ATTEN::Db0
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn is_db2_5(&self) -> bool {
        *self == ITEM15_ATTEN::Db2_5
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn is_db6(&self) -> bool {
        *self == ITEM15_ATTEN::Db6
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn is_db12(&self) -> bool {
        *self == ITEM15_ATTEN::Db12
    }
}
#[doc = "Field `ITEM15_ATTEN` writer - "]
pub type ITEM15_ATTEN_W<'a, REG> = crate::FieldWriter<'a, REG, 2, ITEM15_ATTEN, crate::Safe>;
impl<'a, REG> ITEM15_ATTEN_W<'a, REG>
where
    REG: crate::Writable + crate::RegisterSpec,
    REG::Ux: From<u8>,
{
    #[doc = "No input attenuation, ADC can measure up to approx."]
    #[inline(always)]
    pub fn db0(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM15_ATTEN::Db0)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 2.5 dB"]
    #[inline(always)]
    pub fn db2_5(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM15_ATTEN::Db2_5)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 6 dB"]
    #[inline(always)]
    pub fn db6(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM15_ATTEN::Db6)
    }
    #[doc = "The input voltage of ADC will be attenuated extending the range of measurement by about 12 dB"]
    #[inline(always)]
    pub fn db12(self) -> &'a mut crate::W<REG> {
        self.variant(ITEM15_ATTEN::Db12)
    }
}
#[doc = "Field `ITEM15_CHANNEL` reader - "]
pub type ITEM15_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM15_CHANNEL` writer - "]
pub type ITEM15_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITEM14_ATTEN` reader - "]
pub use ITEM15_ATTEN_R as ITEM14_ATTEN_R;
#[doc = "Field `ITEM14_ATTEN` writer - "]
pub use ITEM15_ATTEN_W as ITEM14_ATTEN_W;
#[doc = "Field `ITEM14_CHANNEL` reader - "]
pub type ITEM14_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM14_CHANNEL` writer - "]
pub type ITEM14_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITEM13_ATTEN` reader - "]
pub use ITEM15_ATTEN_R as ITEM13_ATTEN_R;
#[doc = "Field `ITEM13_ATTEN` writer - "]
pub use ITEM15_ATTEN_W as ITEM13_ATTEN_W;
#[doc = "Field `ITEM13_CHANNEL` reader - "]
pub type ITEM13_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM13_CHANNEL` writer - "]
pub type ITEM13_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
#[doc = "Field `ITEM12_ATTEN` reader - "]
pub use ITEM15_ATTEN_R as ITEM12_ATTEN_R;
#[doc = "Field `ITEM12_ATTEN` writer - "]
pub use ITEM15_ATTEN_W as ITEM12_ATTEN_W;
#[doc = "Field `ITEM12_CHANNEL` reader - "]
pub type ITEM12_CHANNEL_R = crate::FieldReader;
#[doc = "Field `ITEM12_CHANNEL` writer - "]
pub type ITEM12_CHANNEL_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item15_atten(&self) -> ITEM15_ATTEN_R {
        ITEM15_ATTEN_R::new((self.bits & 3) as u8)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn item15_channel(&self) -> ITEM15_CHANNEL_R {
        ITEM15_CHANNEL_R::new(((self.bits >> 2) & 0x0f) as u8)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item14_atten(&self) -> ITEM14_ATTEN_R {
        ITEM14_ATTEN_R::new(((self.bits >> 6) & 3) as u8)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn item14_channel(&self) -> ITEM14_CHANNEL_R {
        ITEM14_CHANNEL_R::new(((self.bits >> 8) & 0x0f) as u8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item13_atten(&self) -> ITEM13_ATTEN_R {
        ITEM13_ATTEN_R::new(((self.bits >> 12) & 3) as u8)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn item13_channel(&self) -> ITEM13_CHANNEL_R {
        ITEM13_CHANNEL_R::new(((self.bits >> 14) & 0x0f) as u8)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item12_atten(&self) -> ITEM12_ATTEN_R {
        ITEM12_ATTEN_R::new(((self.bits >> 18) & 3) as u8)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn item12_channel(&self) -> ITEM12_CHANNEL_R {
        ITEM12_CHANNEL_R::new(((self.bits >> 20) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("SAR2_PATT_TAB4")
            .field("item15_atten", &self.item15_atten())
            .field("item12_atten", &self.item12_atten())
            .field("item12_channel", &self.item12_channel())
            .field("item13_atten", &self.item13_atten())
            .field("item13_channel", &self.item13_channel())
            .field("item14_atten", &self.item14_atten())
            .field("item14_channel", &self.item14_channel())
            .field("item15_channel", &self.item15_channel())
            .finish()
    }
}
impl W {
    #[doc = "Bits 0:1"]
    #[inline(always)]
    pub fn item15_atten(&mut self) -> ITEM15_ATTEN_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM15_ATTEN_W::new(self, 0)
    }
    #[doc = "Bits 2:5"]
    #[inline(always)]
    pub fn item15_channel(&mut self) -> ITEM15_CHANNEL_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM15_CHANNEL_W::new(self, 2)
    }
    #[doc = "Bits 6:7"]
    #[inline(always)]
    pub fn item14_atten(&mut self) -> ITEM14_ATTEN_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM14_ATTEN_W::new(self, 6)
    }
    #[doc = "Bits 8:11"]
    #[inline(always)]
    pub fn item14_channel(&mut self) -> ITEM14_CHANNEL_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM14_CHANNEL_W::new(self, 8)
    }
    #[doc = "Bits 12:13"]
    #[inline(always)]
    pub fn item13_atten(&mut self) -> ITEM13_ATTEN_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM13_ATTEN_W::new(self, 12)
    }
    #[doc = "Bits 14:17"]
    #[inline(always)]
    pub fn item13_channel(&mut self) -> ITEM13_CHANNEL_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM13_CHANNEL_W::new(self, 14)
    }
    #[doc = "Bits 18:19"]
    #[inline(always)]
    pub fn item12_atten(&mut self) -> ITEM12_ATTEN_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM12_ATTEN_W::new(self, 18)
    }
    #[doc = "Bits 20:23"]
    #[inline(always)]
    pub fn item12_channel(&mut self) -> ITEM12_CHANNEL_W<'_, SAR2_PATT_TAB4_SPEC> {
        ITEM12_CHANNEL_W::new(self, 20)
    }
}
#[doc = "Register\n\nYou can [`read`](crate::Reg::read) this register and get [`sar2_patt_tab4::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`sar2_patt_tab4::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct SAR2_PATT_TAB4_SPEC;
impl crate::RegisterSpec for SAR2_PATT_TAB4_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`sar2_patt_tab4::R`](R) reader structure"]
impl crate::Readable for SAR2_PATT_TAB4_SPEC {}
#[doc = "`write(|w| ..)` method takes [`sar2_patt_tab4::W`](W) writer structure"]
impl crate::Writable for SAR2_PATT_TAB4_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets SAR2_PATT_TAB4 to value 0"]
impl crate::Resettable for SAR2_PATT_TAB4_SPEC {}
