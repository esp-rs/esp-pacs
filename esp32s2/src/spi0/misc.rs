#[doc = "Register `MISC` reader"]
pub type R = crate::R<MISC_SPEC>;
#[doc = "Register `MISC` writer"]
pub type W = crate::W<MISC_SPEC>;
#[doc = "Field `CS0_DIS` reader - "]
pub type CS0_DIS_R = crate::BitReader;
#[doc = "Field `CS0_DIS` writer - "]
pub type CS0_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS1_DIS` reader - "]
pub type CS1_DIS_R = crate::BitReader;
#[doc = "Field `CS1_DIS` writer - "]
pub type CS1_DIS_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_END` reader - "]
pub type TRANS_END_R = crate::BitReader;
#[doc = "Field `TRANS_END` writer - "]
pub type TRANS_END_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `TRANS_END_INT_ENA` reader - "]
pub type TRANS_END_INT_ENA_R = crate::BitReader;
#[doc = "Field `TRANS_END_INT_ENA` writer - "]
pub type TRANS_END_INT_ENA_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_POL` reader - "]
pub type CS_POL_R = crate::FieldReader;
#[doc = "Field `CS_POL` writer - "]
pub type CS_POL_W<'a, REG> = crate::FieldWriter<'a, REG, 2>;
#[doc = "Field `FSUB_PIN` reader - "]
pub type FSUB_PIN_R = crate::BitReader;
#[doc = "Field `FSUB_PIN` writer - "]
pub type FSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `SSUB_PIN` reader - "]
pub type SSUB_PIN_R = crate::BitReader;
#[doc = "Field `SSUB_PIN` writer - "]
pub type SSUB_PIN_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CK_IDLE_EDGE` reader - "]
pub type CK_IDLE_EDGE_R = crate::BitReader;
#[doc = "Field `CK_IDLE_EDGE` writer - "]
pub type CK_IDLE_EDGE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `CS_KEEP_ACTIVE` reader - "]
pub type CS_KEEP_ACTIVE_R = crate::BitReader;
#[doc = "Field `CS_KEEP_ACTIVE` writer - "]
pub type CS_KEEP_ACTIVE_W<'a, REG> = crate::BitWriter<'a, REG>;
#[doc = "Field `AUTO_PER` reader - "]
pub type AUTO_PER_R = crate::BitReader;
#[doc = "Field `AUTO_PER` writer - "]
pub type AUTO_PER_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cs0_dis(&self) -> CS0_DIS_R {
        CS0_DIS_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs1_dis(&self) -> CS1_DIS_R {
        CS1_DIS_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trans_end(&self) -> TRANS_END_R {
        TRANS_END_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trans_end_int_ena(&self) -> TRANS_END_INT_ENA_R {
        TRANS_END_INT_ENA_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_pol(&self) -> CS_POL_R {
        CS_POL_R::new(((self.bits >> 5) & 3) as u8)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fsub_pin(&self) -> FSUB_PIN_R {
        FSUB_PIN_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ssub_pin(&self) -> SSUB_PIN_R {
        SSUB_PIN_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_idle_edge(&self) -> CK_IDLE_EDGE_R {
        CK_IDLE_EDGE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cs_keep_active(&self) -> CS_KEEP_ACTIVE_R {
        CS_KEEP_ACTIVE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn auto_per(&self) -> AUTO_PER_R {
        AUTO_PER_R::new(((self.bits >> 11) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("MISC")
            .field("auto_per", &self.auto_per())
            .field("cs_keep_active", &self.cs_keep_active())
            .field("ck_idle_edge", &self.ck_idle_edge())
            .field("ssub_pin", &self.ssub_pin())
            .field("fsub_pin", &self.fsub_pin())
            .field("cs_pol", &self.cs_pol())
            .field("trans_end_int_ena", &self.trans_end_int_ena())
            .field("trans_end", &self.trans_end())
            .field("cs1_dis", &self.cs1_dis())
            .field("cs0_dis", &self.cs0_dis())
            .finish()
    }
}
impl W {
    #[doc = "Bit 0"]
    #[inline(always)]
    pub fn cs0_dis(&mut self) -> CS0_DIS_W<MISC_SPEC> {
        CS0_DIS_W::new(self, 0)
    }
    #[doc = "Bit 1"]
    #[inline(always)]
    pub fn cs1_dis(&mut self) -> CS1_DIS_W<MISC_SPEC> {
        CS1_DIS_W::new(self, 1)
    }
    #[doc = "Bit 3"]
    #[inline(always)]
    pub fn trans_end(&mut self) -> TRANS_END_W<MISC_SPEC> {
        TRANS_END_W::new(self, 3)
    }
    #[doc = "Bit 4"]
    #[inline(always)]
    pub fn trans_end_int_ena(&mut self) -> TRANS_END_INT_ENA_W<MISC_SPEC> {
        TRANS_END_INT_ENA_W::new(self, 4)
    }
    #[doc = "Bits 5:6"]
    #[inline(always)]
    pub fn cs_pol(&mut self) -> CS_POL_W<MISC_SPEC> {
        CS_POL_W::new(self, 5)
    }
    #[doc = "Bit 7"]
    #[inline(always)]
    pub fn fsub_pin(&mut self) -> FSUB_PIN_W<MISC_SPEC> {
        FSUB_PIN_W::new(self, 7)
    }
    #[doc = "Bit 8"]
    #[inline(always)]
    pub fn ssub_pin(&mut self) -> SSUB_PIN_W<MISC_SPEC> {
        SSUB_PIN_W::new(self, 8)
    }
    #[doc = "Bit 9"]
    #[inline(always)]
    pub fn ck_idle_edge(&mut self) -> CK_IDLE_EDGE_W<MISC_SPEC> {
        CK_IDLE_EDGE_W::new(self, 9)
    }
    #[doc = "Bit 10"]
    #[inline(always)]
    pub fn cs_keep_active(&mut self) -> CS_KEEP_ACTIVE_W<MISC_SPEC> {
        CS_KEEP_ACTIVE_W::new(self, 10)
    }
    #[doc = "Bit 11"]
    #[inline(always)]
    pub fn auto_per(&mut self) -> AUTO_PER_W<MISC_SPEC> {
        AUTO_PER_W::new(self, 11)
    }
}
#[doc = "SPI Memory Miscellaneous Register\n\nYou can [`read`](crate::Reg::read) this register and get [`misc::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`misc::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct MISC_SPEC;
impl crate::RegisterSpec for MISC_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`misc::R`](R) reader structure"]
impl crate::Readable for MISC_SPEC {}
#[doc = "`write(|w| ..)` method takes [`misc::W`](W) writer structure"]
impl crate::Writable for MISC_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets MISC to value 0"]
impl crate::Resettable for MISC_SPEC {}
