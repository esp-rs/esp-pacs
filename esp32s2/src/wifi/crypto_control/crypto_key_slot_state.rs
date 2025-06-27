#[doc = "Register `CRYPTO_KEY_SLOT_STATE` reader"]
pub type R = crate::R<CRYPTO_KEY_SLOT_STATE_SPEC>;
#[doc = "Register `CRYPTO_KEY_SLOT_STATE` writer"]
pub type W = crate::W<CRYPTO_KEY_SLOT_STATE_SPEC>;
#[doc = "Field `KEY_SLOT_ENABLE(0-24)` reader - Enable the key slot corresponding to this bit"]
pub type KEY_SLOT_ENABLE_R = crate::BitReader;
#[doc = "Field `KEY_SLOT_ENABLE(0-24)` writer - Enable the key slot corresponding to this bit"]
pub type KEY_SLOT_ENABLE_W<'a, REG> = crate::BitWriter<'a, REG>;
impl R {
    #[doc = "Enable the key slot corresponding to this bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `KEY_SLOT_ENABLE0` field.</div>"]
    #[inline(always)]
    pub fn key_slot_enable(&self, n: u8) -> KEY_SLOT_ENABLE_R {
        #[allow(clippy::no_effect)]
        [(); 25][n as usize];
        KEY_SLOT_ENABLE_R::new(((self.bits >> n) & 1) != 0)
    }
    #[doc = "Iterator for array of:"]
    #[doc = "Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable_iter(&self) -> impl Iterator<Item = KEY_SLOT_ENABLE_R> + '_ {
        (0..25).map(move |n| KEY_SLOT_ENABLE_R::new(((self.bits >> n) & 1) != 0))
    }
    #[doc = "Bit 0 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable0(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new((self.bits & 1) != 0)
    }
    #[doc = "Bit 1 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable1(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 1) & 1) != 0)
    }
    #[doc = "Bit 2 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable2(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 2) & 1) != 0)
    }
    #[doc = "Bit 3 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable3(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 3) & 1) != 0)
    }
    #[doc = "Bit 4 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable4(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 4) & 1) != 0)
    }
    #[doc = "Bit 5 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable5(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 5) & 1) != 0)
    }
    #[doc = "Bit 6 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable6(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 6) & 1) != 0)
    }
    #[doc = "Bit 7 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable7(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 7) & 1) != 0)
    }
    #[doc = "Bit 8 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable8(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 8) & 1) != 0)
    }
    #[doc = "Bit 9 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable9(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 9) & 1) != 0)
    }
    #[doc = "Bit 10 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable10(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 10) & 1) != 0)
    }
    #[doc = "Bit 11 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable11(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 11) & 1) != 0)
    }
    #[doc = "Bit 12 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable12(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 12) & 1) != 0)
    }
    #[doc = "Bit 13 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable13(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 13) & 1) != 0)
    }
    #[doc = "Bit 14 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable14(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 14) & 1) != 0)
    }
    #[doc = "Bit 15 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable15(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 15) & 1) != 0)
    }
    #[doc = "Bit 16 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable16(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 16) & 1) != 0)
    }
    #[doc = "Bit 17 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable17(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 17) & 1) != 0)
    }
    #[doc = "Bit 18 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable18(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 18) & 1) != 0)
    }
    #[doc = "Bit 19 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable19(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 19) & 1) != 0)
    }
    #[doc = "Bit 20 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable20(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 20) & 1) != 0)
    }
    #[doc = "Bit 21 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable21(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 21) & 1) != 0)
    }
    #[doc = "Bit 22 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable22(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 22) & 1) != 0)
    }
    #[doc = "Bit 23 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable23(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 23) & 1) != 0)
    }
    #[doc = "Bit 24 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable24(&self) -> KEY_SLOT_ENABLE_R {
        KEY_SLOT_ENABLE_R::new(((self.bits >> 24) & 1) != 0)
    }
}
#[cfg(feature = "impl-register-debug")]
impl core::fmt::Debug for R {
    fn fmt(&self, f: &mut core::fmt::Formatter) -> core::fmt::Result {
        f.debug_struct("CRYPTO_KEY_SLOT_STATE")
            .field("key_slot_enable0", &self.key_slot_enable0())
            .field("key_slot_enable1", &self.key_slot_enable1())
            .field("key_slot_enable2", &self.key_slot_enable2())
            .field("key_slot_enable3", &self.key_slot_enable3())
            .field("key_slot_enable4", &self.key_slot_enable4())
            .field("key_slot_enable5", &self.key_slot_enable5())
            .field("key_slot_enable6", &self.key_slot_enable6())
            .field("key_slot_enable7", &self.key_slot_enable7())
            .field("key_slot_enable8", &self.key_slot_enable8())
            .field("key_slot_enable9", &self.key_slot_enable9())
            .field("key_slot_enable10", &self.key_slot_enable10())
            .field("key_slot_enable11", &self.key_slot_enable11())
            .field("key_slot_enable12", &self.key_slot_enable12())
            .field("key_slot_enable13", &self.key_slot_enable13())
            .field("key_slot_enable14", &self.key_slot_enable14())
            .field("key_slot_enable15", &self.key_slot_enable15())
            .field("key_slot_enable16", &self.key_slot_enable16())
            .field("key_slot_enable17", &self.key_slot_enable17())
            .field("key_slot_enable18", &self.key_slot_enable18())
            .field("key_slot_enable19", &self.key_slot_enable19())
            .field("key_slot_enable20", &self.key_slot_enable20())
            .field("key_slot_enable21", &self.key_slot_enable21())
            .field("key_slot_enable22", &self.key_slot_enable22())
            .field("key_slot_enable23", &self.key_slot_enable23())
            .field("key_slot_enable24", &self.key_slot_enable24())
            .finish()
    }
}
impl W {
    #[doc = "Enable the key slot corresponding to this bit"]
    #[doc = ""]
    #[doc = "<div class=\"warning\">`n` is number of field in register. `n == 0` corresponds to `KEY_SLOT_ENABLE0` field.</div>"]
    #[inline(always)]
    pub fn key_slot_enable(&mut self, n: u8) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        #[allow(clippy::no_effect)]
        [(); 25][n as usize];
        KEY_SLOT_ENABLE_W::new(self, n)
    }
    #[doc = "Bit 0 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable0(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 0)
    }
    #[doc = "Bit 1 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable1(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 1)
    }
    #[doc = "Bit 2 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable2(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 2)
    }
    #[doc = "Bit 3 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable3(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 3)
    }
    #[doc = "Bit 4 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable4(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 4)
    }
    #[doc = "Bit 5 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable5(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 5)
    }
    #[doc = "Bit 6 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable6(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 6)
    }
    #[doc = "Bit 7 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable7(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 7)
    }
    #[doc = "Bit 8 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable8(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 8)
    }
    #[doc = "Bit 9 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable9(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 9)
    }
    #[doc = "Bit 10 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable10(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 10)
    }
    #[doc = "Bit 11 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable11(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 11)
    }
    #[doc = "Bit 12 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable12(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 12)
    }
    #[doc = "Bit 13 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable13(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 13)
    }
    #[doc = "Bit 14 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable14(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 14)
    }
    #[doc = "Bit 15 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable15(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 15)
    }
    #[doc = "Bit 16 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable16(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 16)
    }
    #[doc = "Bit 17 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable17(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 17)
    }
    #[doc = "Bit 18 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable18(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 18)
    }
    #[doc = "Bit 19 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable19(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 19)
    }
    #[doc = "Bit 20 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable20(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 20)
    }
    #[doc = "Bit 21 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable21(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 21)
    }
    #[doc = "Bit 22 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable22(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 22)
    }
    #[doc = "Bit 23 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable23(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 23)
    }
    #[doc = "Bit 24 - Enable the key slot corresponding to this bit"]
    #[inline(always)]
    pub fn key_slot_enable24(&mut self) -> KEY_SLOT_ENABLE_W<CRYPTO_KEY_SLOT_STATE_SPEC> {
        KEY_SLOT_ENABLE_W::new(self, 24)
    }
}
#[doc = "State of cryptographic key slots\n\nYou can [`read`](crate::Reg::read) this register and get [`crypto_key_slot_state::R`](R). You can [`reset`](crate::Reg::reset), [`write`](crate::Reg::write), [`write_with_zero`](crate::Reg::write_with_zero) this register using [`crypto_key_slot_state::W`](W). You can also [`modify`](crate::Reg::modify) this register. See [API](https://docs.rs/svd2rust/#read--modify--write-api)."]
pub struct CRYPTO_KEY_SLOT_STATE_SPEC;
impl crate::RegisterSpec for CRYPTO_KEY_SLOT_STATE_SPEC {
    type Ux = u32;
}
#[doc = "`read()` method returns [`crypto_key_slot_state::R`](R) reader structure"]
impl crate::Readable for CRYPTO_KEY_SLOT_STATE_SPEC {}
#[doc = "`write(|w| ..)` method takes [`crypto_key_slot_state::W`](W) writer structure"]
impl crate::Writable for CRYPTO_KEY_SLOT_STATE_SPEC {
    type Safety = crate::Unsafe;
}
#[doc = "`reset()` method sets CRYPTO_KEY_SLOT_STATE to value 0"]
impl crate::Resettable for CRYPTO_KEY_SLOT_STATE_SPEC {}
