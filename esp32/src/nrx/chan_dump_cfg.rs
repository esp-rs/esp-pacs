#[doc = "Register `CHAN_DUMP_CFG` reader"]
pub type R = crate::R<CHAN_DUMP_CFG_SPEC>;
#[doc = "Register `CHAN_DUMP_CFG` writer"]
pub type W = crate::W<CHAN_DUMP_CFG_SPEC>;
#[doc = "Field `HTLTF_DUMP_ENABLE` reader - Enable dumping the HT Long Training Field (HT-LTF)"]
pub type HTLTF_DUMP_ENABLE_R = crate::BitReader;
#[doc = "Field `HTLTF_DUMP_ENABLE` writer - Enable dumping the HT Long Training Field (HT-LTF)"]
pub type HTLTF_DUMP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `LLTF_DUMP_ENABLE` reader - Enable dumping the Legacy Long Training Field (LLTF)"]
pub type LLTF_DUMP_ENABLE_R = crate::BitReader;
#[doc = "Field `LLTF_DUMP_ENABLE` writer - Enable dumping the Legacy Long Training Field (LLTF)"]
pub type LLTF_DUMP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `STBC_HTLTF_DUMP_ENABLE` reader - Enable dumping the Space Time Block Coding (STBC) extension HT-LTF"]
pub type STBC_HTLTF_DUMP_ENABLE_R = crate::BitReader;
#[doc = "Field `STBC_HTLTF_DUMP_ENABLE` writer - Enable dumping the Space Time Block Coding (STBC) extension HT-LTF"]
pub type STBC_HTLTF_DUMP_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `MANUAL_SCALING_ENABLE` reader - Enable manually scaling of CSI I/Q samples"]
pub type MANUAL_SCALING_ENABLE_R = crate::BitReader;
#[doc = "Field `MANUAL_SCALING_ENABLE` writer - Enable manually scaling of CSI I/Q samples"]
pub type MANUAL_SCALING_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SAMPLE_SCALING_EXPONENT` reader - Exponent for the scaling factor used by manual I/Q sample scaling"]
pub type SAMPLE_SCALING_EXPONENT_R = crate::FieldReader;
#[doc = "Field `SAMPLE_SCALING_EXPONENT` writer - Exponent for the scaling factor used by manual I/Q sample scaling"]
pub type SAMPLE_SCALING_EXPONENT_W<'a, REG> = crate::FieldWriter<'a, REG, 4>;
impl R {
    #[doc = "Bit 0 - Enable dumping the HT Long Training Field (HT-LTF)"]
    #[inline(always)]
    pub fn htltf_dump_enable(&self) -> HTLTF_DUMP_ENABLE_R {
        HTLTF_DUMP_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable dumping the Legacy Long Training Field (LLTF)"]
    #[inline(always)]
    pub fn lltf_dump_enable(&self) -> LLTF_DUMP_ENABLE_R {
        LLTF_DUMP_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable dumping the Space Time Block Coding (STBC) extension HT-LTF"]
    #[inline(always)]
    pub fn stbc_htltf_dump_enable(&self) -> STBC_HTLTF_DUMP_ENABLE_R {
        STBC_HTLTF_DUMP_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable manually scaling of CSI I/Q samples"]
    #[inline(always)]
    pub fn manual_scaling_enable(&self) -> MANUAL_SCALING_ENABLE_R {
        MANUAL_SCALING_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bits 4:7 - Exponent for the scaling factor used by manual I/Q sample scaling"]
    #[inline(always)]
    pub fn sample_scaling_exponent(&self) -> SAMPLE_SCALING_EXPONENT_R {
        SAMPLE_SCALING_EXPONENT_R::new(((self.bits >> 4) & 0x0f) as u8)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CHAN_DUMP_CFG")
            .field("htltf_dump_enable", &self.htltf_dump_enable())
            .field("lltf_dump_enable", &self.lltf_dump_enable())
            .field("stbc_htltf_dump_enable", &self.stbc_htltf_dump_enable())
            .field("manual_scaling_enable", &self.manual_scaling_enable())
            .field("sample_scaling_exponent", &self.sample_scaling_exponent())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0 - Enable dumping the HT Long Training Field (HT-LTF)"]
    #[inline(always)]
    pub fn htltf_dump_enable(&mut self) -> HTLTF_DUMP_ENABLE_W<CHAN_DUMP_CFG_SPEC> {
        HTLTF_DUMP_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable dumping the Legacy Long Training Field (LLTF)"]
    #[inline(always)]
    pub fn lltf_dump_enable(&mut self) -> LLTF_DUMP_ENABLE_W<CHAN_DUMP_CFG_SPEC> {
        LLTF_DUMP_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable dumping the Space Time Block Coding (STBC) extension HT-LTF"]
    #[inline(always)]
    pub fn stbc_htltf_dump_enable(&mut self) -> STBC_HTLTF_DUMP_ENABLE_W<CHAN_DUMP_CFG_SPEC> {
        STBC_HTLTF_DUMP_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable manually scaling of CSI I/Q samples"]
    #[inline(always)]
    pub fn manual_scaling_enable(&mut self) -> MANUAL_SCALING_ENABLE_W<CHAN_DUMP_CFG_SPEC> {
        MANUAL_SCALING_ENABLE_W::new(self, 3)
    }
    #[doc = "Bits 4:7 - Exponent for the scaling factor used by manual I/Q sample scaling"]
    #[inline(always)]
    pub fn sample_scaling_exponent(&mut self) -> SAMPLE_SCALING_EXPONENT_W<CHAN_DUMP_CFG_SPEC> {
        SAMPLE_SCALING_EXPONENT_W::new(self, 4)
    }
}
#[doc = "Configuration of channel info dumping\n\nYou can [`read`](crate::Reg::read) this register and get [`chan_dump_cfg::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`chan_dump_cfg::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CHAN_DUMP_CFG_SPEC;
impl crate::RegisterSpec for CHAN_DUMP_CFG_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`chan_dump_cfg::R`](R) reader structure"]
impl crate::Readable for CHAN_DUMP_CFG_SPEC {}
#[doc = "`write(|w| ..)` method takes [`chan_dump_cfg::W`](W) writer structure"]
impl crate::Writable for CHAN_DUMP_CFG_SPEC {
    type Safety = crate::Unsafe;
    const ZERO_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
    const ONE_TO_MODIFY_FIELDS_BITMAP: u32 = 0;
}
#[doc = "`reset()` method sets CHAN_DUMP_CFG to value 0"]
impl crate::Resettable for CHAN_DUMP_CFG_SPEC {
    const RESET_VALUE: u32 = 0;
}
